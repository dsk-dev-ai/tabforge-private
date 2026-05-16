use super::audio::{
    initialize_audio
};

use super::video::{
    initialize_video
};

use super::router::{
    initialize_router
};

pub fn initialize_stream(){

    println!(
        "[CAPTURE] Stream engine online"
    );

    let _audio=
        initialize_audio();

    let _video=
        initialize_video();

    let _router=
        initialize_router();

    println!(
        "[CAPTURE] Multi-tab pipeline active"
    );

    println!(
        "[CAPTURE] Audio/video sync enabled"
    );

}