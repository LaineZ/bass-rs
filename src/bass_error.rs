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
    /// The sample buffer was lost
    BufLost,
    /// Invalid handle
    Handle,
    /// Unsupported sample format
    Format,
    /// Invalid playback position
    Position,
    /// BASS_Init has not been successfully called
    Init,
    /// BASS_Start has not been successfully called
    Start,

    ///No CD in drive
    Nocd,
    ///Invalid track number
    Cdtrack,
    ///Already initialized/paused/whatever
    Already,
    ///Not paused
    Nopause,
    ///Not an audio track
    Notaudio,
    ///Can't get a free channel
    Nochan,
    ///An illegal type was specified
    Illtype,
    ///An illegal parameter was specified
    Illparam,
    ///No 3D support
    No3d,
    ///No EAX support
    Noeax,
    ///Illegal device number
    Device,
    ///Not playing
    Noplay,
    ///Illegal sample rate
    Freq,
    ///The stream is not a file stream
    Notfile,
    ///No hardware voices available
    Nohw,
    ///The MOD music has no sequence data
    Empty,
    ///No internet connection could be opened
    Nonet,
    ///Couldn't create the file
    Create,
    ///Effects are not available
    Nofx,
    ///The channel is playing
    Playing,
    ///Requested data is not available
    Notavail,
    ///The channel is a 'decoding channel'
    Decode,
    ///A sufficient DirectX version is not installed
    Dx,
    ///Connection timedout
    Timeout,
    ///Unsupported file format
    Fileform,
    ///Unavailable speaker
    Speaker,
    ///Invalid BASS version (used by add-ons)
    Version,
    ///Codec is not available/supported
    Codec,
    ///The channel/file has ended
    Ended,
    ///The device is busy (eg. in "exclusive" use by another process)
    Busy,
    ///BassWma: the file is protected
    WmaLicense,
    ///BassWma: WM9 is required
    WmaWm9,
    ///BassWma: access denied (user/pass is invalid)
    WmaDenied,
    ///BassWma: no appropriate codec is installed
    WmaCodec,
    ///BassWma: individualization is needed
    WmaIndividual,
    ///BassEnc: ACM codec selection cancelled
    AcmCancel,
    ///BassEnc: Access denied (invalid password)
    CastDenied,
    ///BASSWASAPI: no WASAPI available
    Wasapi,
    ///BASS_AAC: non-streamable due to MP4 atom order ('mdat' before 'moov')
    Mp4Nostream,

    ///Some other mystery error
    Unknown(i32),
}
impl BassError {
    pub fn from_code(bass_err:i32) -> Self {
        match bass_err {

            other => BassError::Unknown(other)
        }
    }
}
