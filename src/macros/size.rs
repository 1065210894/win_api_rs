#[macro_export]
/// Get's size of either value or type.
/// To get size of value prefix variable ident with `@`.
/// ```
/// let a: u32 = 5u32;
/// assert_eq!(enlit::size_of!(@a), 4);
/// assert_eq!(enlit::size_of!(u128), 16);
/// ```
macro_rules! size_of {
    ($type:ty) => {
        std::mem::size_of::<$type>()
    };
    (@$var:ident) => {
        std::mem::size_of_val(&$var)
    };
}