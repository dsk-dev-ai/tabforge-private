use crate::capture::stream::initialize_stream;

use crate::encoder::ffmpeg::initialize_ffmpeg;
use crate::encoder::hardware::initialize_hardware;

use crate::ipc::socket::{initialize_socket_runtime, listen_for_streams, start_websocket_server};

use crate::workers::pool::initialize_workers;

use tokio::runtime::Runtime;

pub fn initialize_runtime_services() {
    println!();

    println!("========== RUNTIME BOOT ==========");

    /*
    ---------------------------------
    CAPTURE SYSTEM
    ---------------------------------
    */

    initialize_stream();

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

    /*
    ---------------------------------
    TOKIO RUNTIME
    ---------------------------------
    */

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    runtime.spawn(async {
        listen_for_streams().await;
    });

    runtime.spawn(async {
        start_websocket_server().await;
    });

    /*
    keep runtime alive
    */

    std::mem::forget(runtime);

    /*
    ---------------------------------
    FINAL STATUS
    ---------------------------------
    */

    println!("[BOOT] Runtime initialized");

    println!("[BOOT] Multi-tab recording ready");

    println!("[BOOT] WebSocket bridge active");

    println!("[BOOT] Real browser ingest enabled");

    println!("[BOOT] FFmpeg mux pipeline active");

    println!("[BOOT] Hardware acceleration enabled");

    println!("==================================");

    println!();
}
