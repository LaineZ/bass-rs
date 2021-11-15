use std::ops::Deref;
use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct SampleChannel {
    handle: Arc<u32>,
    newest_channel: Channel
}
impl SampleChannel {

    fn new(handle: u32) -> BassResult<Self> {
        let mut sc = Self {
            handle: Arc::new(handle),
            newest_channel: Channel::new(0)
        };
        sc.get_channel()?;

        Ok(sc)
    }

    pub fn sample_from_mem(data: &Vec<u8>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        Self::new(check_bass_err!(BASS_SampleLoad(
            true.ibool(), 
            data.as_ptr() as *const std::ffi::c_void, 
            offset.into_len(), 
            data.len() as u32, 
            max_channels, 
            0
        )))
    }

    pub fn get_channel(&mut self) -> BassResult<Channel> {
        self.newest_channel = Channel::new(check_bass_err!(BASS_SampleGetChannel(*self.handle, false.ibool())));
        Ok(self.newest_channel.clone())
    }
}

impl Deref for SampleChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.newest_channel
    }
}
