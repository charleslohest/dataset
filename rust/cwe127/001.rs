const BUFFER_SIZE: usize = 32;

#[repr(C)]
struct Frame {
    secret: [u8; BUFFER_SIZE],
    buffer: [u8; BUFFER_SIZE],
}

fn vuln(index: i32, size: i32) -> Result<String, ()> {
    let f = Frame {
        secret: {
            let tmp = b"guacamole\0";
            let mut buffer: [u8; BUFFER_SIZE] = [b'\0'; BUFFER_SIZE];
            buffer[..tmp.len()].copy_from_slice(tmp);
            buffer
        },
        buffer: {
            let tmp = b"Lorem ipsum dolor sit amet, con\0";
            let mut buffer: [u8; BUFFER_SIZE] = [b' '; BUFFER_SIZE];
            buffer[..tmp.len()].copy_from_slice(tmp);
            buffer
        },
    };

    if index + size < 32 && index < 32 {
        let mut out = Vec::with_capacity(size as usize);

        unsafe {
            // dbg!("here", std::ffi::CStr::from_ptr(f.buffer.as_ptr() as *const i8));
            std::ptr::copy_nonoverlapping(
                f.buffer.as_ptr().offset(index as isize),
                out.as_mut_ptr(),
                size as usize,
            );
        }
        // dbg!(*f.buffer.as_ptr().offset(index as isize));
        // }
        // dbg!(&out);
        return Ok(unsafe {
            std::ffi::CStr::from_ptr(out.as_mut_ptr() as *const i8)
                .to_str()
                .unwrap()
                .to_string()
        });
    }

    return Err(());
}

fn main() {
    dbg!(vuln(5, 10));
    dbg!(vuln(-32, 10));
}
