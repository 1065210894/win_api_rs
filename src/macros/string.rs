#[macro_export]
/// Converts [[u8]] or [[i8]] to &str.
/// ```
/// use enlit::to_c_str;
///
/// assert_eq!(
///     "Hello World!",
///     unsafe {
///         to_c_str!(b"Hello World!")
///     }
/// );
///
/// ```
macro_rules! to_c_str {
    ($array:expr) => {
        std::ffi::CStr::from_ptr($array.as_ptr() as *const i8)
            .to_string_lossy()
            .as_ref()
    };
}