use std::{ffi::c_void, sync::Arc};

use bass_sys::*;

use crate::{bass_error::*, check_bass_err, check_bass_err_val, check_bass_err_bool};

#[derive(Clone)]
pub struct Channel {
    channel_id: Arc<u32>
}
impl Channel {
    pub fn create_stream_mem<O:Into<u64>>(bytes: &Vec<u8>, offset: O) -> BassResult<Self> {

        // create the stream
        let pointer = bass_sys::BASS_StreamCreateFile(
            1,
            bytes.as_ptr() as *const c_void,
            offset.into(),
            bytes.len() as u64,
            0
        );
        // check for an error when creating the stream
        check_bass_err!(pointer);

        
        // double check the channel is valid
        let mut temp_info = BassChannelInfo::new(
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            vec![0i8].as_ptr()
        );
        check_bass_err!(BASS_ChannelGetInfo(pointer, &mut temp_info));

        // should be good to go from here
        let channel_id = Arc::new(pointer);

        Ok(Self {
            channel_id
        })
    }


    pub fn get_attribute(&self, attrib: ChannelAttribute) -> BassResult<f32> {
        let mut value = 0.0;
        check_bass_err!(BASS_ChannelGetAttribute(*self.channel_id, attrib as u32, &mut value));
        Ok(value)
    }
    pub fn set_attribute(&self, attrib: ChannelAttribute, value: f32) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetAttribute(*self.channel_id, attrib as u32, value));
        Ok(())
    }


    pub fn get_position(&self) -> BassResult<u64> {
        let pos = BASS_ChannelGetPosition(*self.channel_id, BASS_POS_BYTE);
        check_bass_err_val!(pos, u64::MAX);
        Ok(pos)
    }
    pub fn set_position<P:Into<u64>>(&self, pos:P) -> BassResult<()> {
        check_bass_err!(BASS_ChannelSetPosition(*self.channel_id, pos.into(), BASS_POS_BYTE));
        Ok(())
    }


    pub fn bytes2seconds<P:Into<u64>>(&self, pos: P) -> BassResult<f64> {
        let secs = BASS_ChannelBytes2Seconds(*self.channel_id, pos.into());
        check_bass_err_bool!(secs < 0.0);
        Ok(secs)
    }
    pub fn seconds2bytes<R:From<u64>>(&self, pos: f64) -> BassResult<R> {
        let r = BASS_ChannelSeconds2Bytes(*self.channel_id, pos);
        check_bass_err_val!(r, u64::MAX);
        Ok(r.into())
    }


    pub fn play(&self, restart:bool) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPlay(*self.channel_id, restart.ibool()));
        Ok(())
    }
    pub fn pause(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelPause(*self.channel_id));
        Ok(())
    }
    pub fn stop(&self) -> BassResult<()> {
        check_bass_err!(BASS_ChannelStop(*self.channel_id));
        Ok(())
    }
}

pub enum ChannelAttribute {

}

impl Drop for Channel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.channel_id);
        if count == 0 {
            // need to free the bass channel
            BASS_StreamFree(*self.channel_id);
        }
    }
}



trait IntoIBool {
    fn ibool(&self) -> i32;
}
impl IntoIBool for bool {
    fn ibool(&self) -> i32 {
        if *self {1} else {0}
    }
}