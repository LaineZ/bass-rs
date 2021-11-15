#![allow(unused_variables, dead_code)]
use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct Channel {
    // pub(crate) 
    pub handle: Arc<u32>
}
impl Channel {
    pub fn new(handle: u32) -> Self {
        Self {
            handle: Arc::new(handle)
        }
    }

    pub fn get_attribute(&self, attrib: ChannelAttribute) -> BassResult<f32> {
        let mut value = 0.0;
        check_bass_err!(BASS_ChannelGetAttribute(*self.handle, attrib.into(), &mut value));
        Ok(value)
    }
    pub fn set_attribute(&self, attrib: ChannelAttribute, value: f32) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetAttribute(*self.handle, attrib.into(), value));
        Ok(())
    }


    pub fn get_position(&self) -> BassResult<f64> {
        let pos = check_bass_err_val!(BASS_ChannelGetPosition(*self.handle, BASS_POS_BYTE), u64::MAX);
        let secs = self.bytes2seconds(pos)?;
        Ok(secs)
    }
    pub fn set_position(&self, secs:f64) -> BassResult<()> {
        let pos = self.seconds2bytes(secs)?.into_len();
        check_bass_err!(BASS_ChannelSetPosition(*self.handle, pos, BASS_POS_BYTE));
        Ok(())
    }


    pub fn bytes2seconds(&self, pos: impl IntoLen) -> BassResult<f64> {
        let secs = BASS_ChannelBytes2Seconds(*self.handle, pos.into_len());
        check_bass_err_bool!(secs < 0.0);
        Ok(secs)
    }
    pub fn seconds2bytes(&self, secs: f64) -> BassResult<impl IntoLen> {
        Ok(check_bass_err_val!(BASS_ChannelSeconds2Bytes(*self.handle, secs), u64::MAX))
    }


    pub fn play(&self, restart:bool) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPlay(*self.handle, restart.ibool()));
        Ok(())
    }
    pub fn pause(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPause(*self.handle));
        Ok(())
    }
    pub fn stop(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelStop(*self.handle));
        Ok(())
    }


    // convenience functions
    pub fn get_volume(&self) -> BassResult<f32> {
        self.get_attribute(ChannelAttribute::Volume)
    }
    pub fn set_volume(&self, vol: f32) -> BassResult<()> {
        self.set_attribute(ChannelAttribute::Volume, vol)
    }
}
impl Drop for Channel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            // need to free the bass channel
            if BASS_StreamFree(*self.handle) == 0 {
                panic!("error dropping stream")
            }
        }
    }
}
