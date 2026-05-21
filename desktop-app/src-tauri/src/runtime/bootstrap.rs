use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use tokio::{runtime::Runtime, time::sleep};

use crate::capture::stream::initialize_stream;

use crate::encoder::ffmpeg::initialize_ffmpeg;

use crate::encoder::hardware::initialize_hardware;

use crate::ipc::socket::{initialize_socket_runtime, listen_for_streams, start_websocket_server};

use crate::workers::pool::{initialize_workers, worker_stats};

// =====================================
// GLOBAL
// =====================================

static RUNTIME_READY: OnceLock<bool> = OnceLock::new();

// =====================================
// INIT
// =====================================

pub fn initialize_runtime_services() {
    let boot = Instant::now();

    println!();

    println!("========== RUNTIME BOOT ==========");

    // =====================================
    // CAPTURE
    // =====================================

    println!("[BOOT] Capture");

    initialize_stream();

    // =====================================
    // ENCODER
    // =====================================

    println!("[BOOT] Encoder");

    initialize_ffmpeg();

    initialize_hardware();

    // =====================================
    // WORKERS
    // =====================================

    println!("[BOOT] Workers");

    initialize_workers();

    // =====================================
    // IPC
    // =====================================

    println!("[BOOT] IPC");

    initialize_socket_runtime();

    // =====================================
    // TOKIO
    // =====================================

    println!("[BOOT] Tokio");

    let runtime = Runtime::new().expect("tokio init failed");

    runtime.spawn(async {
        listen_for_streams().await;
    });

    runtime.spawn(async {
        start_websocket_server().await;
    });

    // background runtime health

    runtime.spawn(async {
        loop {
            println!("[RUNTIME HEARTBEAT]");

            sleep(Duration::from_secs(30)).await;
        }
    });

    // worker monitor

    runtime.spawn(async {
        loop {
            worker_stats();

            sleep(Duration::from_secs(15)).await;
        }
    });

    RUNTIME_READY.set(true).ok();

    // keep runtime alive

    std::mem::forget(runtime);

    // =====================================
    // FINAL
    // =====================================

    println!();

    println!("[BOOT OK]");

    println!("[BOOT] Multi-tab ready");

    println!("[BOOT] Browser ingest active");

    println!("[BOOT] FFmpeg active");

    println!("[BOOT] Worker routing active");

    println!("[BOOT] WebSocket online");

    println!("[BOOT] Hardware acceleration enabled");

    println!("[BOOT TIME] {:?}", boot.elapsed());

    println!("==================================");

    println!();
}

// =====================================
// DEBUG
// =====================================

pub fn runtime_alive() -> bool {
    RUNTIME_READY.get().copied().unwrap_or(false)
}
