use crate::capture::stream::
    initialize_stream;

use crate::encoder::ffmpeg::
    initialize_ffmpeg;

use crate::encoder::hardware::
    initialize_hardware;

use crate::ipc::socket::{
    initialize_socket_runtime,
    listen_for_streams
};

use crate::workers::pool::
    initialize_workers;



pub fn initialize_runtime_services(){

    println!();

    println!(
        "========== RUNTIME BOOT =========="
    );


    /*
    CAPTURE
    */

    initialize_stream();


    /*
    ENCODER
    */

    initialize_ffmpeg();

    initialize_hardware();


    /*
    WORKERS
    */

    initialize_workers();


    /*
    IPC
    */

    initialize_socket_runtime();

    listen_for_streams();


    println!(
        "[BOOT] Runtime initialized"
    );

    println!(
        "[BOOT] Multi-tab recording ready"
    );

    println!(
        "[BOOT] WebSocket bridge active"
    );

    println!(
        "[BOOT] Per-tab audio isolation enabled"
    );

    println!(
        "[BOOT] FFmpeg mux pipeline active"
    );

    println!(
        "[BOOT] Hardware acceleration enabled"
    );

    println!(
        "=================================="
    );

    println!();

}