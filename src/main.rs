use std::io::Error;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use winapi::um::winuser::{MessageBoxW, MB_OK};
use winapi::um;

fn main() {
    print_message("你好").unwrap();
}

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe { MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK) };
    if ret == 0 {
        Err(Error::last_os_error())
    } else {
        Ok(ret)
    }
}