#![allow(unused_variables, dead_code, non_snake_case)]
use std::{collections::HashMap};

use once_cell::sync::Lazy;

pub type BassResult<T> = Result<T, BassError>;
const ERROR_MAP: Lazy<HashMap<i32, BassError>> = Lazy::new(|| {
    use BassError::*;
    use bass_sys::*;

    HashMap::from([
        (BASS_OK, BassError::Ok),
        (BASS_ERROR_MEM, Mem),
        (BASS_ERROR_FILEOPEN, FileOpen),
        (BASS_ERROR_DRIVER, Driver),
        (BASS_ERROR_BUFLOST, BufLost),
        (BASS_ERROR_HANDLE, Handle),
        (BASS_ERROR_FORMAT, Format),
        (BASS_ERROR_POSITION, Position),
        (BASS_ERROR_INIT, Init),
        (BASS_ERROR_START, Start),
        (BASS_ERROR_ALREADY, Already),
        (BASS_ERROR_NOTAUDIO, Notaudio),
        (BASS_ERROR_NOCHAN, Nochan),
        (BASS_ERROR_ILLTYPE, Illtype),
        (BASS_ERROR_ILLPARAM, Illparam),
        (BASS_ERROR_NO3D, No3d),
        (BASS_ERROR_NOEAX, Noeax),
        (BASS_ERROR_DEVICE, Device),
        (BASS_ERROR_NOPLAY, Noplay),
        (BASS_ERROR_FREQ, Freq),
        (BASS_ERROR_NOTFILE, Notfile),
        (BASS_ERROR_NOHW, Nohw),
        (BASS_ERROR_EMPTY, Empty),
        (BASS_ERROR_NONET, Nonet),
        (BASS_ERROR_CREATE, Create),
        (BASS_ERROR_NOFX, Nofx),
        (BASS_ERROR_NOTAVAIL, Notavail),
        (BASS_ERROR_DECODE, Decode),
        (BASS_ERROR_DX, Dx),
        (BASS_ERROR_TIMEOUT, Timeout),
        (BASS_ERROR_FILEFORM, Fileform),
        (BASS_ERROR_SPEAKER, Speaker),
        (BASS_ERROR_VERSION, Version),
        (BASS_ERROR_CODEC, Codec),
        (BASS_ERROR_ENDED, Ended),
        (BASS_ERROR_BUSY, Busy),
    ])
});

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
        if let Some(&err) = ERROR_MAP.get(&bass_err) {
            err
        } else {
            Self::Unknown(bass_err)
        }
    }
}
