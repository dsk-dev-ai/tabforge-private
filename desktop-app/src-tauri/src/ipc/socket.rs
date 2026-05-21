use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use futures_util::StreamExt;

use tokio::net::{TcpListener, TcpStream};

use tokio_tungstenite::accept_async;

use uuid::Uuid;

use serde::Deserialize;

use crate::encoder::ffmpeg::{create_muxer, stop_muxer, write_chunk};

use crate::workers::pool::{assign_worker, release_worker};

// ======================================
// TYPES
// ======================================

type SessionStore = Arc<Mutex<HashMap<String, StreamSession>>>;

#[derive(Debug, Default)]

pub struct StreamSession {
    pub session_id: String,

    pub worker: Option<u32>,

    pub chunks: u64,

    pub total_bytes: usize,

    pub active: bool,

    pub started: u64,
}

#[derive(Debug, Deserialize)]

struct MetaPacket {
    sessionId: String,

    tabId: String,

    chunkIndex: u64,

    timestamp: u64,

    size: usize,

    type_field: String,
}

// ======================================
// INIT
// ======================================

pub fn initialize_socket_runtime() {
    println!();

    println!("========== IPC ==========");

    println!("[IPC] Socket runtime initialized");

    println!("[IPC] WebSocket enabled");

    println!("[IPC] Binary routing enabled");

    println!("[IPC] Worker dispatch active");

    println!("=========================");

    println!();
}

pub async fn listen_for_streams() {
    println!("[IPC] Awaiting extension...");

    println!("[IPC] ws://127.0.0.1:8765");
}

// ======================================
// SERVER
// ======================================

pub async fn start_websocket_server() {
    let listener = match TcpListener::bind("127.0.0.1:8765").await {
        Ok(v) => v,

        Err(error) => {
            println!("[BIND ERROR] {}", error);

            return;
        }
    };

    println!("[WS] Server active");

    let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let sessions = Arc::clone(&sessions);

                tokio::spawn(async move {
                    handle_connection(stream, sessions).await;
                });
            }

            Err(error) => {
                println!("[CONNECT ERROR] {}", error);
            }
        }
    }
}

// ======================================
// CONNECTION
// ======================================

async fn handle_connection(stream: TcpStream, sessions: SessionStore) {
    let ws = match accept_async(stream).await {
        Ok(v) => v,

        Err(error) => {
            println!("[UPGRADE ERROR] {}", error);

            return;
        }
    };

    let (_, mut read) = ws.split();

    let tab_id = format!("tab_{}", Uuid::new_v4());

    println!("[SESSION START] {}", tab_id);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(message) => {
                if message.is_close() {
                    shutdown_session(&tab_id, &sessions);

                    break;
                }

                if message.is_ping() {
                    continue;
                }

                if message.is_text() {
                    let text = message.into_text().unwrap_or_default();

                    println!("[META] {}", text);

                    continue;
                }

                if message.is_binary() {
                    let bytes = message.into_data();

                    process_chunk(&tab_id, bytes, &sessions);
                }
            }

            Err(error) => {
                println!("[STREAM ERROR] {}", error);

                shutdown_session(&tab_id, &sessions);

                break;
            }
        }
    }
}

// ======================================
// PROCESS
// ======================================

fn process_chunk(tab_id: &str, bytes: Vec<u8>, sessions: &SessionStore) {
    let mut store = match sessions.lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[LOCK ERROR]");

            return;
        }
    };

    let session = store.entry(tab_id.to_string()).or_insert_with(|| {
        let worker = assign_worker(tab_id);

        create_muxer(tab_id, &format!("recordings/{}.webm", tab_id));

        StreamSession {
            session_id: tab_id.to_string(),

            worker,

            chunks: 0,

            total_bytes: 0,

            active: true,

            started: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    });

    session.chunks += 1;

    session.total_bytes += bytes.len();

    write_chunk(tab_id, &bytes);

    if session.chunks % 10 == 0 {
        println!();

        println!("[STREAM] {}", tab_id);

        println!("worker {:?}", session.worker);

        println!("chunks {}", session.chunks);

        println!("bytes {}KB", session.total_bytes / 1024);

        println!("active {}", session.active);

        println!("----------------");
    }
}

// ======================================
// SHUTDOWN
// ======================================

fn shutdown_session(tab_id: &str, sessions: &SessionStore) {
    let mut store = match sessions.lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    store.remove(tab_id);

    drop(store);

    release_worker(tab_id);

    stop_muxer(tab_id);

    println!("[SESSION CLOSED] {}", tab_id);
}
