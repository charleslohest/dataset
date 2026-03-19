#![feature(rustc_private)]
#![feature(ip_as_octets)]

use std::{net::ToSocketAddrs, time::Duration};

extern crate libc;
use libc::socket;

unsafe extern "C" {
    // there is no equivalent to this function in rust
    fn gethostbyaddr(
        addr: *const libc::c_void,
        len: libc::socklen_t,
        type_: libc::c_int,
    ) -> *mut libc::hostent;

}

fn str_to_ipaddress(addr: &str) -> Result<std::net::Ipv4Addr, std::num::ParseIntError> {
    // removed the buffer overflow at buffer
    let buffer = addr.clone().to_string(); // by default Rust uses the heap when stack is
                                           // not required, we could use the stack here
                                           // as well but, this is not the main vuln
                                           // from the source so leave it as is
    let mut acc: [u8; 4] = [0; 4];

    for (i, c) in buffer.split(".").enumerate() {
        acc[i] = c.parse()?;
    }

    Ok(acc.into())
}

// cannot use threads if not a Send/Sync structure
struct HostentWrapper {
    ptr: *mut libc::hostent,
}

// SAFETY: Caller must ensure no data races or invalid pointer usage
unsafe impl Send for HostentWrapper {}
unsafe impl Sync for HostentWrapper {}

fn vuln(user_supplied_addr: String) -> Result<(), ()> {
    if let Ok(addr) = str_to_ipaddress(&user_supplied_addr) {
        let (tx, rx) = std::sync::mpsc::channel();
        let thread = std::thread::spawn(move || {
            let hp = unsafe {
                let addr = addr.to_bits();
                gethostbyaddr(
                    &addr as *const u32 as *const libc::c_void,
                    core::mem::size_of::<u32>() as libc::socklen_t,
                    libc::AF_INET,
                )
            };
            tx.send(()).unwrap();
            HostentWrapper { ptr: hp } // kind of hacky
        });

        let _ = rx.recv_timeout(Duration::from_secs(3)).expect(&format!(
            "[timeout] could not resolve the host for address : {}",
            user_supplied_addr
        ));

        let hp = thread.join().unwrap().ptr;
        let hostname = unsafe { std::ffi::CStr::from_ptr((*hp).h_name as *const libc::c_char) }
            .to_str()
            .expect("failed to convert to a str")
            .to_string();
        println!("the host is {}", hostname);
    }

    Ok(())
}

fn main() {
    vuln("8.8.8.8".to_string());
    vuln("192.168.1.1".to_string()); // intented use */
    vuln("81.138.71.238.81.138.71.238.138.71.238.81.138.71.238.138.71.238.81.138.71.238.138.71.238.81.138.71.238".to_string()); // vuln
    vuln("65465464.465464.644654.6456465".to_string()); // vuln */
    vuln("81.138.71.238".to_string()); // fixed
}
