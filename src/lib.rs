use bass_error::BassResult;

// mods
mod macros;
mod channel;
mod bass_error;
mod bass_flags;


pub fn init_default() -> BassResult<()> {
    bass_sys::BASS_Init(-1, bass_sys::BASS_DEVICE_STEREO, 0, 0 as *mut std::ffi::c_void, 0 as *mut std::ffi::c_void);
    Ok(())
}

mod test {
    #[test]
    pub fn test() {
        macro_rules! check {
            ($res:expr) => {
                if let Err(e) = $res {
                    panic!("Bass error: {:?}", e);
                } else {
                    $res.unwrap()
                }
            };
        }

        // copy the bass lib file into the exec dir
        move_dll();

        // init bass
        check!(crate::init_default());

        // read test file
        let file_path = "./test.mp3";
        let bytes = std::fs::read(file_path).expect("Error reading ./test.mp3");

        // create stream
        let stream = check!(crate::channel::Channel::create_stream_mem(&bytes, 0u64));

        // try playing
        check!(stream.play(false));

        // give it some time to play
        loop {}
    }


    fn move_dll() {
        #[cfg(target_os = "windows")]
        let filename = "bass.dll";
           
        #[cfg(target_os = "linux")]
        let filename = "libbass.so";

        #[cfg(target_os = "macos")]
        let filename = "libbass.dylib";

        if let Ok(mut library_path) = std::env::current_exe() {
            library_path.pop();
            library_path.push(filename);

            println!("lib dir: {:?}", library_path);
            if library_path.exists() {return}

            std::fs::copy(filename, library_path).expect("error copying lib to exe path");
        } else {
            panic!("error with current dir for lib")
        }
    }
}