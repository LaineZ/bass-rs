pub trait ToBassFlags<T> {
    fn to_flags(&self) -> Vec<T>;
}
pub trait AsBassFlags {
    fn to_num(self) -> u32;
}
impl<I:Iterator> AsBassFlags for I where I::Item: Into<u32> + Copy {
    fn to_num(self) -> u32 {
        let mut total = 0;

        for i in self {
            total += i.into()
        }
        total
    }
}