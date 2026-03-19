const SZ: usize = 32;

fn vuln() {
    let layout = match std::alloc::Layout::from_size_align(SZ, 1) {
        Ok(l) => l,
        Err(_) => {
            panic!("failed to allocate the layout")
        }
    };

    let culprit = unsafe { std::alloc::alloc(layout) };
    unsafe { std::alloc::dealloc(culprit, layout) };

    let arr = unsafe { std::alloc::alloc(layout) };
    unsafe { std::ptr::write_bytes(arr, b'A', SZ) };

    if unsafe { culprit.add(4).read() } == b'A' {
        println!("use-after-free observed");
    } else {
        
        println!("reuse did not occur (still UAF)");
    }
}

fn main() {
    vuln();
}
