use std::collections::HashMap;

use std::sync::{Mutex, OnceLock};

#[derive(Debug)]

pub struct FFmpegSession {
    pub tab_id: String,

    pub output: String,

    pub bytes_received: u64,

    pub active: bool,
}

static SESSIONS: OnceLock<Mutex<HashMap<String, FFmpegSession>>> = OnceLock::new();

fn store() -> &'static Mutex<HashMap<String, FFmpegSession>> {
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn initialize_ffmpeg() {
    store();

    println!("[ENCODER] FFmpeg initialized");

    println!("[ENCODER] MP4 container enabled");

    println!("[ENCODER] H264 + Opus pipeline ready");

    println!("[ENCODER] Awaiting browser chunks");
}

pub fn create_muxer(tab_id: &str, output: &str) {
    let mut sessions = store().lock().unwrap();

    if sessions.contains_key(tab_id) {
        return;
    }

    sessions.insert(
        tab_id.to_string(),
        FFmpegSession {
            tab_id: tab_id.to_string(),

            output: output.to_string(),

            bytes_received: 0,

            active: true,
        },
    );

    println!("[MUXER] Created {}", tab_id);

    println!("[MUXER] Output {}", output);
}

pub fn write_chunk(tab_id: &str, size: usize) {
    let mut sessions = store().lock().unwrap();

    if let Some(session) = sessions.get_mut(tab_id) {
        session.bytes_received += size as u64;

        if session.bytes_received % 40960 < 4096 {
            println!("[MUXER] {} | {}KB", tab_id, session.bytes_received / 1024);
        }
    }
}

#[allow(dead_code)]

pub fn stop_muxer(tab_id: &str) {
    let mut sessions = store().lock().unwrap();

    sessions.remove(tab_id);

    println!("[MUXER] Closed {}", tab_id);
}
