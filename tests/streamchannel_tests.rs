use bass::prelude::BassResult;

pub fn stream_channel_tests() -> BassResult<()> {

    // read test file
    let stream = {
        let file_path = "./test.mp3";
        let bytes = std::fs::read(file_path).expect("Error reading ./test.mp3");

        // create stream
        let stream = check!(StreamChannel::create_from_memory(bytes, 0));
        stream
    };

    // check!(stream.set_attribute(ChannelAttribute::MusicSpeed, 2.0));

    // try playing
    check!(stream.play(false));

    check!(stream.set_position(20.0));
    check!(stream.set_volume(0.1));
    // check!(stream.set_attribute(ChannelAttribute::Pan, -1.0));

    let current_freq = check!(stream.get_attribute(ChannelAttribute::Frequency));
    check!(stream.set_attribute(ChannelAttribute::Frequency, current_freq * 1.7));

    // println!("pan: {}", check!(stream.get_attribute(ChannelAttribute::Pan)));


    // let stream2 = stream.clone();
    // drop(stream);

    // give it some time to play
    loop {}
}