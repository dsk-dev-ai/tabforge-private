use crate::capture::audio::initialize_audio;

use crate::capture::router::initialize_router;

use crate::capture::stream::initialize_stream;

use crate::capture::video::initialize_video;

use crate::encoder::ffmpeg::initialize_ffmpeg;

use crate::encoder::hardware::initialize_hardware;

use crate::ipc::socket::{initialize_socket_runtime, listen_for_streams};

use crate::workers::pool::initialize_workers;

pub fn initialize_runtime_services() {
    println!();

    println!("========== RUNTIME BOOT ==========");

    /*
    ---------------------------------
    CAPTURE SYSTEM
    ---------------------------------
    */

    initialize_stream();

    initialize_audio();

    initialize_video();

    initialize_router();

    /*
    ---------------------------------
    ENCODER SYSTEM
    ---------------------------------
    */

    initialize_ffmpeg();

    initialize_hardware();

    /*
    ---------------------------------
    WORKER SYSTEM
    ---------------------------------
    */

    initialize_workers();

    /*
    ---------------------------------
    IPC SYSTEM
    ---------------------------------
    */

    initialize_socket_runtime();

    listen_for_streams();

    /*
    ---------------------------------
    FINAL STATUS
    ---------------------------------
    */

    println!("[BOOT] Runtime initialized");

    println!("[BOOT] Multi-tab recording ready");

    println!("[BOOT] Live chunk transport active");

    println!("[BOOT] Per-tab audio isolation enabled");

    println!("[BOOT] Hardware acceleration enabled");

    println!("==================================");

    println!();
}
