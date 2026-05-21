use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use uuid::Uuid;

use crate::encoder::ffmpeg::{create_muxer, stop_muxer, write_chunk};

type SessionStore = Arc<Mutex<HashMap<String, StreamSession>>>;

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
    println!();
    println!("========== IPC ==========");

    println!("[IPC] Socket runtime initialized");

    println!("[IPC] WebSocket transport enabled");

    println!("[IPC] Binary streaming enabled");

    println!("[IPC] Multi-tab routing active");

    println!("[IPC] Browser ingest enabled");

    println!("=========================");
    println!();
}

pub async fn listen_for_streams() {
    println!("[IPC] Awaiting extension...");

    println!("[IPC] ws://127.0.0.1:8765");
}

pub async fn start_websocket_server() {
    let listener = match TcpListener::bind("127.0.0.1:8765").await {
        Ok(socket) => socket,

        Err(error) => {
            println!("[IPC ERROR] {}", error);

            return;
        }
    };

    println!("[WS] Server active");

    let sessions: SessionStore = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let connection = listener.accept().await;

        let (stream, _) = match connection {
            Ok(data) => data,

            Err(error) => {
                println!("[CONNECTION ERROR] {}", error);

                continue;
            }
        };

        println!("[BROWSER] Connected");

        let sessions = Arc::clone(&sessions);

        tokio::spawn(async move {
            handle_connection(stream, sessions).await;
        });
    }
}

async fn handle_connection(stream: tokio::net::TcpStream, sessions: SessionStore) {
    let websocket = match accept_async(stream).await {
        Ok(ws) => ws,

        Err(error) => {
            println!("[WS ERROR] {}", error);

            return;
        }
    };

    let (_, mut read) = websocket.split();

    let tab_id = format!("tab_{}", Uuid::new_v4());

    println!("[SESSION START] {}", tab_id);

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_close() {
                    shutdown_session(&tab_id, &sessions);

                    break;
                }

                if msg.is_ping() {
                    continue;
                }

                if msg.is_text() {
                    println!("[META] {}", tab_id);

                    continue;
                }

                if msg.is_binary() {
                    let bytes = msg.into_data();

                    process_chunk(&tab_id, bytes.len(), &sessions);
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

fn process_chunk(tab_id: &str, size: usize, sessions: &SessionStore) {
    let mut store = sessions.lock().unwrap();

    let session = store.entry(tab_id.to_string()).or_default();

    session.active = true;

    session.chunks += 1;

    session.total_bytes += size;

    if session.chunks == 1 {
        create_muxer(tab_id, &format!("recordings/{}.mp4", tab_id));
    }

    write_chunk(tab_id, size);

    if session.chunks % 10 == 0 {
        println!();

        println!("[STREAM] {}", tab_id);

        println!("Chunks : {}", session.chunks);

        println!("Bytes : {} KB", session.total_bytes / 1024);

        println!("Active : {}", session.active);

        println!("----------------");
    }
}

fn shutdown_session(tab_id: &str, sessions: &SessionStore) {
    let mut store = sessions.lock().unwrap();

    store.remove(tab_id);

    drop(store);

    stop_muxer(tab_id);

    println!("[SESSION CLOSED] {}", tab_id);
}

#[allow(dead_code)]

pub fn send_message(message: SocketMessage) {
    println!("[OUTBOUND]");

    println!("Tab : {}", message.tab_id);

    println!("Bytes : {}", message.payload.len());
}
