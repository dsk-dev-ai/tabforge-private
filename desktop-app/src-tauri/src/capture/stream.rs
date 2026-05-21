use super::audio::initialize_audio;
use super::router::initialize_router;
use super::video::initialize_video;

pub fn initialize_stream() {
    println!("[CAPTURE] Stream engine online");

    initialize_audio();

    initialize_video();

    initialize_router();
}
