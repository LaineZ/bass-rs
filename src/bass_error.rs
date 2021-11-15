
pub type BassResult<T> = Result<T, BassError>;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BassError {
    /// all is ok
    Ok,
    /// Memory error
    Mem,
    ///	Can't open the file
    FileOpen,
    /// Can't find a free/valid driver
    Driver,

    BufLost,


}
impl BassError {
    pub fn from_bass_err(bass_err:i32) -> Self {
        todo!()
    }
}