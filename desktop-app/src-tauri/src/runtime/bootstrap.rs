use std::{
    sync::OnceLock,
    time::{Duration,Instant}
};

use tokio::{
    runtime::Runtime,
    time::sleep
};

use crate::capture::stream::
initialize_stream;

use crate::encoder::ffmpeg::{
    initialize_ffmpeg,
    encoder_stats,
    list_sessions
};

use crate::encoder::hardware::
initialize_hardware;

use crate::ipc::socket::{
    initialize_socket_runtime,
    listen_for_streams,
    start_websocket_server
};

use crate::workers::pool::{
    initialize_workers,
    worker_stats,
    list_workers
};


// ====================================
// GLOBAL
// ====================================

static RUNTIME_READY:

OnceLock<bool>

=OnceLock::new();



// ====================================
// HEALTH
// ====================================

async fn runtime_heartbeat(){

loop{

println!(
"[RUNTIME HEARTBEAT]"
);

sleep(

Duration::from_secs(
30
)

)

.await;

}

}



// ====================================
// WORKERS
// ====================================

async fn worker_monitor(){

loop{

worker_stats();

sleep(

Duration::from_secs(
20
)

)

.await;

}

}



// ====================================
// ENCODER
// ====================================

async fn encoder_monitor(){

loop{

encoder_stats();

sleep(

Duration::from_secs(
25
)

)

.await;

}

}



// ====================================
// SESSIONS
// ====================================

async fn session_monitor(){

loop{

list_sessions();

sleep(

Duration::from_secs(
35
)

)

.await;

}

}



// ====================================
// WORKER DETAIL
// ====================================

async fn worker_detail(){

loop{

list_workers();

sleep(

Duration::from_secs(
45
)

)

.await;

}

}



// ====================================
// INIT
// ====================================

pub fn initialize_runtime_services(){

let boot=

Instant::now();


println!();

println!(
"========== TABFORGE =========="
);

println!(
"[BOOT] Runtime"
);


// ====================================
// CAPTURE
// ====================================

println!(
"[BOOT] Capture"
);

initialize_stream();


// ====================================
// ENCODER
// ====================================

println!(
"[BOOT] Encoder"
);

initialize_ffmpeg();

initialize_hardware();


// ====================================
// WORKERS
// ====================================

println!(
"[BOOT] Workers"
);

initialize_workers();


// ====================================
// IPC
// ====================================

println!(
"[BOOT] IPC"
);

initialize_socket_runtime();


// ====================================
// TOKIO
// ====================================

println!(
"[BOOT] Tokio"
);


let runtime=

Runtime::new()

.expect(

"tokio failed"

);


runtime.spawn(

async{

listen_for_streams()

.await;

}

);


runtime.spawn(

async{

start_websocket_server()

.await;

}

);


// monitors

runtime.spawn(
runtime_heartbeat()
);

runtime.spawn(
worker_monitor()
);

runtime.spawn(
encoder_monitor()
);

runtime.spawn(
session_monitor()
);

runtime.spawn(
worker_detail()
);


// ready

RUNTIME_READY

.set(true)

.ok();


// keep runtime

std::mem::forget(
runtime
);


// ====================================
// COMPLETE
// ====================================

println!();

println!(
"[BOOT OK]"
);

println!(
"[BOOT] Browser ingest active"
);

println!(
"[BOOT] Native session runtime"
);

println!(
"[BOOT] FFmpeg ownership active"
);

println!(
"[BOOT] Worker routing active"
);

println!(
"[BOOT] WebSocket online"
);

println!(
"[BOOT] Hardware acceleration active"
);

println!(
"[BOOT] Session monitors active"
);

println!(

"[BOOT TIME] {:?}",

boot.elapsed()

);

println!(
"=============================="
);

println!();

}



// ====================================
// STATUS
// ====================================

pub fn runtime_alive()

->bool{

RUNTIME_READY

.get()

.copied()

.unwrap_or(
false
)

}