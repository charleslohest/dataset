struct MetaData {
    name: [u8; 32],
    version: [u8; 8],
}

#[repr(C)]
union MyUnion {
    metadata: std::mem::ManuallyDrop<MetaData>,
    buffer: [u8; const { core::mem::size_of::<MetaData>() }],
}

fn vuln() {
    // in rust you cannot allocate N bytes and use n st. n < N directly
    let mut p = MyUnion {
        metadata: std::mem::ManuallyDrop::new(MetaData {
            name: {
                let tmp = b"I love this code";
                let mut buffer: [u8; 32] = [b' '; 32];
                buffer[..tmp.len()].copy_from_slice(tmp);
                buffer
            },
            version: {
                let tmp = b"1";
                let mut buffer: [u8; 8] = [b' '; 8];
                buffer[..tmp.len()].copy_from_slice(tmp);
                buffer
            },
        }),
    };

    let start = unsafe { p.metadata.name.iter().position(|x| *x == b"l"[0]).unwrap() };
    let overwrite = 32 - start;

    let padding_size = (overwrite - 14) as usize; // + 1 in rust
    let padding = vec![0; padding_size];

    let new = [
        &{
            let tmp = b"hate this code";
            let mut buffer: [u8; 14] = [b' '; 14];
            buffer[..tmp.len()].copy_from_slice(tmp);
            buffer
        }[..],
        padding.as_slice(),
        b"2",
    ]
    .concat();

    unsafe {
        std::ptr::copy_nonoverlapping(
            new.as_ptr(),
            p.buffer.as_mut_ptr().add(start),
            overwrite + 8,
        );
    }

    unsafe {
        // for i in p.buffer.iter() {
        //     dbg!(i);
        // }
        for i in p.metadata.version {
            dbg!(i);
        }
        println!("{:?}", std::ffi::CStr::from_bytes_until_nul(p.metadata.name.as_slice()));
        println!("{:?}", std::ffi::CStr::from_bytes_until_nul(p.metadata.version.as_slice()));
        // dbg!(str::from_utf8(p.metadata.version.as_slice()));
    }

    // char padding[padding_size];
    // memset((void *)padding, ' ', padding_size);
    // padding[padding_size - 1] = '\0';
}

fn main() {
    vuln();
}
