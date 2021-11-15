#![allow(unused_variables, dead_code)]
use std::sync::Arc;

use crate::prelude::*;

#[derive(Clone)]
pub struct Channel {
    pub(crate) handle: Arc<u32>
}
impl Channel {
    pub fn new(handle: u32) -> Self {
        Self {
            handle: Arc::new(handle)
        }
    }

    pub fn get_attribute(&self, attrib: ChannelAttribute) -> BassResult<f32> {
        let mut value = 0.0;
        check_bass_err!(BASS_ChannelGetAttribute(*self.handle, attrib as u32, &mut value));
        Ok(value)
    }
    pub fn set_attribute(&self, attrib: ChannelAttribute, value: f32) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetAttribute(*self.handle, attrib as u32, value));
        Ok(())
    }


    pub fn get_position(&self) -> BassResult<u64> {
        let pos = BASS_ChannelGetPosition(*self.handle, BASS_POS_BYTE);
        check_bass_err_val!(pos, u64::MAX);
        Ok(pos)
    }
    pub fn set_position<P:Into<u64>>(&self, pos:P) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetPosition(*self.handle, pos.into(), BASS_POS_BYTE));
        Ok(())
    }


    pub fn bytes2seconds<P:Into<u64>>(&self, pos: P) -> BassResult<f64> {
        let secs = BASS_ChannelBytes2Seconds(*self.handle, pos.into());
        check_bass_err_bool!(secs < 0.0);
        Ok(secs)
    }
    pub fn seconds2bytes<R:From<u64>>(&self, pos: f64) -> BassResult<R> {
        let r = BASS_ChannelSeconds2Bytes(*self.handle, pos);
        check_bass_err_val!(r, u64::MAX);
        Ok(r.into())
    }


    pub fn play(&self, restart:bool) -> BassResult<()> {
        println!("not music");
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
}
impl Drop for Channel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.handle);
        if count == 1 {
            // need to free the bass channel
            BASS_StreamFree(*self.handle);
        }
    }
}