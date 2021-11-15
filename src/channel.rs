
use std::{ffi::c_void, sync::Arc};

use crate::bass_error::*;

pub struct Channel {
    channel_id: Arc<u32>
}
impl Channel {
    pub fn create_stream() -> BassResult<Self> {

        let bytes = vec![0u8];

        let pointer = unsafe {
            bass_sys::BASS_StreamCreateFile(
                1, 
                bytes.as_ptr() as *const c_void,
                0, 
                bytes.len() as u64, 
                0
            )
        };
        
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        let count = Arc::<u32>::strong_count(&self.channel_id);
        if count == 0 {
            
        }
    }
}