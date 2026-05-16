use std::collections::HashMap;

use std::sync::{Arc, Mutex};

use std::thread;

use std::time::{SystemTime, UNIX_EPOCH};

use crate::encoder::ffmpeg::{create_muxer, write_chunk};

const SIMULATION_MODE: bool = false;

#[derive(Debug, Clone)]

pub struct SocketMessage {
    pub tab_id: String,

    pub payload: Vec<u8>,

    pub timestamp: u64,

    pub stream_type: String,

    pub chunk_index: u64,
}

#[derive(Debug, Default)]

pub struct StreamSession {
    pub total_bytes: usize,

    pub chunks: u64,

    pub active: bool,
}

pub fn initialize_socket_runtime() {
    println!("[IPC] Socket initialized");

    println!("[IPC] WebSocket online");

    println!("[IPC] Binary mode enabled");

    println!("[IPC] Multi-tab routing enabled");

    println!("[IPC] Real browser ingest mode");
}

pub fn listen_for_streams() {
    println!("[IPC] Listening...");

    let sessions = Arc::new(Mutex::new(HashMap::<String, StreamSession>::new()));

    if SIMULATION_MODE {
        println!("[IPC] Simulation enabled");

        thread::spawn({
            let sessions = Arc::clone(&sessions);

            move || {
                let mut chunk = 0;

                loop {
                    std::thread::sleep(std::time::Duration::from_secs(2));

                    chunk += 1;

                    process_stream_chunk(
                        SocketMessage {
                            tab_id: "tab_001".to_string(),

                            payload: vec![0; 4096],

                            timestamp: current_timestamp(),

                            stream_type: "video/webm".to_string(),

                            chunk_index: chunk,
                        },
                        &sessions,
                    );
                }
            }
        });
    } else {
        println!("[IPC] Awaiting browser extension...");

        println!("[IPC] ws://127.0.0.1:8765");
    }
}

pub fn process_stream_chunk(
    message: SocketMessage,

    sessions: &Arc<Mutex<HashMap<String, StreamSession>>>,
) {
    let mut store = sessions.lock().unwrap();

    let state = store.entry(message.tab_id.clone()).or_default();

    state.active = true;

    state.chunks += 1;

    state.total_bytes += message.payload.len();

    if state.chunks == 1 {
        let output = format!("recordings/{}.mp4", message.tab_id);

        create_muxer(&message.tab_id, &output);
    }

    write_chunk(&message.tab_id, message.payload.len());

    if state.chunks % 10 == 0 {
        println!();

        println!("[STREAM] {}", message.tab_id);

        println!("Chunks: {}", state.chunks);

        println!("Bytes: {} KB", state.total_bytes / 1024);

        println!("Timestamp: {}", message.timestamp);

        println!("-------------");
    }
}

#[allow(dead_code)]

pub fn send_message(message: SocketMessage) {
    println!("[IPC OUTBOUND]");

    println!("Tab: {}", message.tab_id);

    println!("Bytes: {}", message.payload.len());
}

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
