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

    let mut buffer = String::new();
    let mut culprit: *mut u8 = std::ptr::null_mut();
    let mut array: *mut u8 = std::ptr::null_mut();
    loop {
        std::io::stdin().read_line(&mut buffer).unwrap();
        dbg!(buffer.trim());

        match buffer.trim() {
            "kill culprit" => unsafe {
                std::alloc::dealloc(culprit, layout);
            },
            "init culprit" => unsafe {
                culprit = std::alloc::alloc(layout);
            },
            "get culprit" => unsafe {
                for i in 0..SZ {
                    print!("{} ", *culprit.add(i));
                    println!("")
                }
            },
            "kill array" => unsafe {
                std::alloc::dealloc(array, layout);

            },
            "init array" => unsafe {
                array = std::alloc::alloc(layout);
                unsafe { std::ptr::write_bytes(array, b'A', SZ) };
            },
            "get array" => unsafe {
                for i in 0..SZ {
                    print!("{} ", *array.add(i));
                    println!("")
                }
            },
            _ => {
                println!("unknown command");
            }
        }
        
        buffer.clear();
    }
}

fn main() {
    vuln();
}
