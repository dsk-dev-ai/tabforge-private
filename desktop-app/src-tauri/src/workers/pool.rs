use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

// ====================================
// WORKER
// ====================================

#[derive(Debug, Clone)]

pub struct Worker {
    pub id: u32,

    pub active: bool,

    pub busy: bool,

    pub sessions: u64,

    pub current_tab: Option<String>,

    pub started: u64,

    pub bytes_processed: u64,
}

// ====================================
// POOL
// ====================================

static WORKERS: OnceLock<Mutex<HashMap<u32, Worker>>> = OnceLock::new();

fn pool() -> &'static Mutex<HashMap<u32, Worker>> {
    WORKERS.get_or_init(|| Mutex::new(HashMap::new()))
}

// ====================================
// TIME
// ====================================

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

// ====================================
// INIT
// ====================================

pub fn initialize_workers() {
    let mut workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[WORKER LOCK ERROR]");

            return;
        }
    };

    workers.clear();

    for id in 1..=6 {
        workers.insert(
            id,
            Worker {
                id,

                active: true,

                busy: false,

                sessions: 0,

                current_tab: None,

                started: now(),

                bytes_processed: 0,
            },
        );
    }

    println!("[WORKERS] initialized");

    println!("[WORKERS] {} online", workers.len());
}

// ====================================
// ASSIGN
// least busy worker strategy
// ====================================

pub fn assign_worker(tab_id: &str) -> Option<u32> {
    let mut workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => {
            println!("[LOCK ERROR]");

            return None;
        }
    };

    let selected = workers
        .values_mut()
        .filter(|w| w.active && !w.busy)
        .min_by_key(|w| w.sessions);

    if let Some(worker) = selected {
        worker.busy = true;

        worker.sessions += 1;

        worker.current_tab = Some(tab_id.to_string());

        println!("[ASSIGNED] worker={} tab={}", worker.id, tab_id);

        return Some(worker.id);
    }

    println!("[NO WORKER] {}", tab_id);

    None
}

// ====================================
// RELEASE
// ====================================

pub fn release_worker(tab_id: &str) {
    let mut workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    for worker in workers.values_mut() {
        if worker.current_tab.as_deref() == Some(tab_id) {
            worker.busy = false;

            worker.current_tab = None;

            println!("[RELEASE] worker={} tab={}", worker.id, tab_id);

            return;
        }
    }
}

// ====================================
// TRACK
// ====================================

pub fn add_bytes(worker_id: u32, bytes: u64) {
    let mut workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    if let Some(worker) = workers.get_mut(&worker_id) {
        worker.bytes_processed += bytes;
    }
}

// ====================================
// DEBUG
// ====================================

pub fn list_workers() {
    let workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    println!();

    for worker in workers.values() {
        println!(
            "[WORKER] id={} active={} busy={} sessions={} bytes={}KB",
            worker.id,
            worker.active,
            worker.busy,
            worker.sessions,
            worker.bytes_processed / 1024
        );
    }

    println!();
}

// ====================================
// STATS
// ====================================

pub fn worker_stats() {
    let workers = match pool().lock() {
        Ok(v) => v,

        Err(_) => return,
    };

    let total = workers.len();

    let busy = workers.values().filter(|w| w.busy).count();

    let idle = total - busy;

    println!();

    println!("[POOL]");

    println!("total={}", total);

    println!("busy={}", busy);

    println!("idle={}", idle);

    println!();
}
