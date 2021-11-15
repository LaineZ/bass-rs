
pub(crate) trait IBool {
    fn ibool(&self) -> i32;
}
impl IBool for bool {
    fn ibool(&self) -> i32 {
        if *self {1} else {0}
    }
}

/// helper for lengths/positions
pub trait Len {
    fn len(&self) -> u64;
}
macro_rules! __impl_len {
    ($($type:ty),+) => {
        $(
            impl Len for $type {
                fn len(&self) -> u64 {
                    *self as u64
                }
            }
        )+
    };
}
__impl_len!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);
