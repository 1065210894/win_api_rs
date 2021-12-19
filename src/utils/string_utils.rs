use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;

/**
将字符串变成 unicode编码 双字节数组 并获取指针
 */
pub fn get_string16_ptr(str: &str) -> *const u16 {
    let wide: Vec<u16> = OsStr::new(str).encode_wide().chain(once(0)).collect();
    wide.as_ptr()
}


