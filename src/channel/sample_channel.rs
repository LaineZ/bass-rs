use std::ops::Deref;
use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct SampleChannel {
    handle: Arc<u32>,
    channels: Vec<Channel>,
    newest_channel: Channel,
    _data: Arc<Vec<u8>>
}
impl SampleChannel {
    fn new(handle: u32, data: Vec<u8>) -> BassResult<Self> {
        let mut sc = Self {
            handle: Arc::new(handle),
            channels: Vec::new(),
            newest_channel: Channel::new(0),
            _data: Arc::new(data)
        };
        sc.get_channel()?;

        Ok(sc)
    }

    pub fn load_from_memory(data: Vec<u8>, offset: impl IntoLen, max_channels: u32) -> BassResult<Self> {
        Self::new(check_bass_err!(BASS_SampleLoad(
            true.ibool(), 
            data.as_ptr() as *const c_void, 
            offset.into_len(), 
            data.len() as u32, 
            max_channels, 
            BASS_SAMPLE_OVER_POS
        )), data)
    }

    pub fn get_channel(&mut self) -> BassResult<Channel> {
        self.newest_channel = Channel::new(check_bass_err!(BASS_SampleGetChannel(*self.handle, false.ibool() as u32)));
        if !self.channels.contains(&self.newest_channel) {
            self.channels.push(self.newest_channel.clone());
        }
        Ok(self.newest_channel.clone())
    }
}
impl Drop for SampleChannel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            // need to free the bass channel
            if BASS_SampleFree(*self.handle) == 0 {
                panic!("error dropping sample")
            }
        }
    }
}

impl Deref for SampleChannel {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.newest_channel
    }
}
