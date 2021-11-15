#[test]
pub fn test() {
    use bass::channel::*;

    macro_rules! check {
        ($res:expr) => {
            match $res {
                Err(e) => panic!("Bass error: {:?}", e),
                Ok(e) => e
            }
        };
    }

    // copy the bass lib file into the exec dir
    move_dll();

    // init bass
    check!(bass::init_default());

    // read test file
    let file_path = "./test.mp3";
    let bytes = std::fs::read(file_path).expect("Error reading ./test.mp3");

    // create stream
    let stream = check!(StreamChannel::create_from_memory(&bytes, 0));

    // check!(stream.set_attribute(ChannelAttribute::MusicSpeed, 2.0));

    // try playing
    check!(stream.play(false));

    check!(stream.set_position(20.0));
    check!(stream.set_volume(0.1));
    check!(stream.set_attribute(ChannelAttribute::Pan, -1.0));
    // println!("pan: {}", check!(stream.get_attribute(ChannelAttribute::Pan)));


    // let stream2 = stream.clone();
    // drop(stream);

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