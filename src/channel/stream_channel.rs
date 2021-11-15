use std::ops::Deref;
use std::ffi::c_void;

use crate::prelude::*;

#[derive(Clone)]
pub struct StreamChannel {
    pub channel: Channel
}
impl StreamChannel {
    pub fn create_from_memory(bytes: &Vec<u8>, offset: impl IntoLen) -> BassResult<Self> {
        // create the stream
        let handle = bass_sys::BASS_StreamCreateFile(
            1,
            bytes.as_ptr() as *const c_void,
            offset.into_len(),
            bytes.len() as u64,
            0
        );
        // check for an error when creating the stream
        check_bass_err!(handle);

        // double check the channel is valid
        check_bass_err!(bass_sys::BASS_ChannelGetInfo(handle, &mut new_channel_info()));

        // should be good to go from here
        Ok(Self {
            channel: Channel::new(handle)
        })
    }

}
impl Deref for StreamChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.channel
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