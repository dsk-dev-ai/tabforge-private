use super::audio::initialize_audio;
use super::video::initialize_video;
use super::router::initialize_router;

pub fn initialize_stream() {

    println!(
        "[CAPTURE] Stream engine online"
    );

    initialize_audio();

    initialize_video();

    initialize_router();
}