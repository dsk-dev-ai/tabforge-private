use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::Path,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

// ==========================================
// GLOBAL METRICS
// ==========================================

#[derive(Debug, Default)]

pub struct EncoderMetrics {
    pub sessions_started: u64,

    pub sessions_closed: u64,

    pub total_chunks: u64,

    pub total_bytes: u64,

    pub write_errors: u64,
}

// ==========================================
// SESSION
// ==========================================

#[derive(Debug)]

pub struct FFmpegSession {
    pub tab_id: String,

    pub output: String,

    pub bytes_received: u64,

    pub chunks: u64,

    pub active: bool,

    pub started: u64,

    pub file: Option<BufWriter<File>>,
}

// ==========================================
// GLOBAL STATE
// ==========================================

static SESSIONS: OnceLock<Mutex<HashMap<String, FFmpegSession>>> = OnceLock::new();

static METRICS: OnceLock<Mutex<EncoderMetrics>> = OnceLock::new();

fn store() -> &'static Mutex<HashMap<String, FFmpegSession>> {
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

fn metrics() -> &'static Mutex<EncoderMetrics> {
    METRICS.get_or_init(|| Mutex::new(EncoderMetrics::default()))
}

// ==========================================
// INIT
// ==========================================

pub fn initialize_ffmpeg() {
    store();

    metrics();

    create_dir_all("recordings").ok();

    println!("[ENCODER] Initialized");

    println!("[ENCODER] Recording dir ready");

    println!("[ENCODER] Multi-session enabled");

    println!("[ENCODER] Buffered writer enabled");
}

// ==========================================
// CREATE
// ==========================================

pub fn create_muxer(tab_id: &str, output: &str) {
    let mut sessions = match store().lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[MUX] Lock poisoned");

            return;
        }
    };

    if sessions.contains_key(tab_id) {
        return;
    }

    if let Some(parent) = Path::new(output).parent() {
        create_dir_all(parent).ok();
    }

    let file = match File::create(output) {
        Ok(file) => Some(BufWriter::new(file)),

        Err(error) => {
            println!("[MUX ERROR] {}", error);

            None
        }
    };

    let started = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    sessions.insert(
        tab_id.to_string(),
        FFmpegSession {
            tab_id: tab_id.to_string(),

            output: output.to_string(),

            bytes_received: 0,

            chunks: 0,

            active: true,

            started,

            file,
        },
    );

    if let Ok(mut m) = metrics().lock() {
        m.sessions_started += 1;
    }

    println!("[MUX CREATED] {}", tab_id);

    println!("[OUTPUT] {}", output);
}

// ==========================================
// WRITE
// ==========================================

pub fn write_chunk(tab_id: &str, data: &[u8]) {
    let mut sessions = match store().lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[WRITE] Lock poisoned");

            return;
        }
    };

    if let Some(session) = sessions.get_mut(tab_id) {
        if !session.active {
            return;
        }

        session.chunks += 1;

        session.bytes_received += data.len() as u64;

        if let Ok(mut m) = metrics().lock() {
            m.total_chunks += 1;

            m.total_bytes += data.len() as u64;
        }

        if let Some(file) = session.file.as_mut() {
            if let Err(error) = file.write_all(data) {
                println!("[WRITE ERROR] {}", error);

                if let Ok(mut m) = metrics().lock() {
                    m.write_errors += 1;
                }
            }
        }

        if session.chunks % 10 == 0 {
            println!(
                "[MUX] {} | chunks={} | {}KB",
                tab_id,
                session.chunks,
                session.bytes_received / 1024
            );
        }
    }
}

// ==========================================
// STOP
// ==========================================

pub fn stop_muxer(tab_id: &str) {
    let mut sessions = match store().lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[STOP] Lock poisoned");

            return;
        }
    };

    if let Some(mut session) = sessions.remove(tab_id) {
        session.active = false;

        if let Some(writer) = session.file.as_mut() {
            let _ = writer.flush();
        }

        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - session.started;

        if let Ok(mut m) = metrics().lock() {
            m.sessions_closed += 1;
        }

        println!("[MUX CLOSED] {}", tab_id);

        println!("[DURATION] {}s", duration);

        println!("[BYTES] {}KB", session.bytes_received / 1024);

        println!("[CHUNKS] {}", session.chunks);
    }
}

// ==========================================
// DEBUG
// ==========================================

pub fn list_sessions() {
    let sessions = match store().lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    for (id, session) in sessions.iter() {
        println!(
            "[SESSION] {} active={} chunks={} bytes={}",
            id, session.active, session.chunks, session.bytes_received
        );
    }
}

pub fn encoder_stats() {
    if let Ok(m) = metrics().lock() {
        println!("[ENCODER STATS]");

        println!("started={}", m.sessions_started);

        println!("closed={}", m.sessions_closed);

        println!("chunks={}", m.total_chunks);

        println!("bytes={}KB", m.total_bytes / 1024);

        println!("errors={}", m.write_errors);
    }
}
