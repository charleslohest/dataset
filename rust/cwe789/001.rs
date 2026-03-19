use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ffi::CString;

fn proc_msg(s: &str, msg_len: i32) -> i32 {
    let pre_len: i32 = "preamble: ".len() as i32;
    let buf_len: i32 = pre_len - msg_len;

    println!(
        "[proc_msg] pre_len={}, msg_len={}, buf_len={}",
        pre_len, msg_len, buf_len
    );

    // CWE-789: uncontrolled allocation size
    if buf_len <= 0 {
        // Simulate C behavior: negative becomes huge when cast to usize
        println!("[proc_msg] negative buf_len, will underflow");
    }

    let alloc_size = buf_len as usize; // ❌ underflow if buf_len < 0

    let layout = match Layout::from_size_align(alloc_size, 1) {
        Ok(l) => l,
        Err(_) => {
            println!("[proc_msg] invalid layout");
            return -1;
        }
    };

    unsafe {
        let buf = alloc(layout);
        if buf.is_null() {
            println!("[proc_msg] malloc failed (likely huge allocation)");
            return -1;
        }

        // Dummy processing (like strncpy)
        let bytes = s.as_bytes();
        let copy_len = usize::min(bytes.len(), alloc_size);
        ptr::copy_nonoverlapping(bytes.as_ptr(), buf, copy_len);

        if alloc_size > 0 {
            *buf.add(alloc_size - 1) = 0;
        }

        let cstr = CString::from_raw(buf as *mut i8);
        println!("[proc_msg] buf=\"{:?}\"", cstr);

        // prevent double-free
        let _ = cstr.into_raw();
        dealloc(buf, layout);
    }

    0
}

fn vuln() {
    let s = "preamble: message\n";
    let msg_len = s.find(':').unwrap_or(0) as i32;

    println!("[safe_call] msg_len={}", msg_len);
    proc_msg(s, msg_len);

    let vuln = "preamble: message\n";
    let msg_len = 100;

    println!("[vuln_call] msg_len={}", msg_len);
    proc_msg(vuln, msg_len);
}

fn main() {
    vuln();
}
