use std::ops::Deref;
use std::sync::Arc;

use crate::prelude::*;

/// ## Stream channel. 
/// 
/// Use this if you want to play audio from memory
/// 
/// See [`here`](https://www.un4seen.com/doc/#bass/BASS_StreamCreate.html) for more information
/// 
/// # Usage
/// 
/// Create a new stream channel with [`StreamChannel::create_from_memory`](#method.create_from_memory)
/// 
/// See [`Channel`] for further documentation
/// 
/// # Dropping
/// See [`Channel`] for drop behaviour
#[derive(Clone)]
pub struct StreamChannel {
    pub channel: Channel,

    /// needed so the data stays in memory while its needed by bass
    _data: Arc<Vec<u8>>
}
impl StreamChannel {
    /// Create a StreamChannel from bytes in memory
    /// ```
    /// let bytes = std::fs::read(path.as_ref())?;
    /// let channel = StreamChannel::create_from_memory(bytes, 0i32).expect("Error creating stream channel")
    /// channel.play().expect("error playing channel");
    /// ```
    pub fn create_from_memory(bytes: Vec<u8>, offset: impl IntoLen) -> BassResult<Self> {
        // create the stream
        let handle = bass_sys::BASS_StreamCreateFile(
            1,
            bytes.as_ptr() as *const c_void,
            offset.into_len(),
            bytes.len() as u64,
            BASS_STREAM_PRESCAN
        );
        // check for an error when creating the stream
        check_bass_err!(handle);

        // double check the channel is valid
        check_bass_err!(bass_sys::BASS_ChannelGetInfo(handle, &mut new_channel_info()));

        // should be good to go from here
        Ok(Self {
            channel: Channel::new(handle),
            _data: Arc::new(bytes)
        })
    }

    // pub fn create(freq: u64, ) -> BassResult<Self> {
    //     BASS_StreamCreate(freq, channels, flags, )
    // }

}
impl Deref for StreamChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.channel
    }
}
impl Drop for StreamChannel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            // need to free the bass channel
            if BASS_StreamFree(*self.channel.handle) == 0 {
                panic!("error dropping stream: {:?}", BassError::get_last_error())
            }
        }
    }
}

#[inline]
fn new_channel_info() -> BassChannelInfo {
    BassChannelInfo::new(
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        vec![0i8].as_ptr()
    )
}