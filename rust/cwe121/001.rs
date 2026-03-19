const BUFFER_SIZE: usize = 10;

#[unsafe(no_mangle)]
fn vuln(pwd: &str) {
    dbg!(&pwd);
    let mut buf_pwd = [0u8; BUFFER_SIZE];
    let mut is_password_good: bool = false;
    unsafe {
        std::ptr::copy_nonoverlapping(pwd.as_ptr(), buf_pwd.as_mut_ptr(), pwd.len());
    }

    // hard coded credential
    if str::from_utf8(&buf_pwd).unwrap() == "7c076e55-a9f1-4689-84c7-be825e3e1be4" {
        is_password_good = true;
    }

    if is_password_good {
        println!("you found my cookie jar!");
    } else {
        println!("my cookies are safe :)");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    vuln(&args[1]);
}
