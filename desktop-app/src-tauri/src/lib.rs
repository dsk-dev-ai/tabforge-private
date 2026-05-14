mod recorder;
mod encoder;
mod ipc;
mod workers;
mod models;

use ipc::bridge::{receive_tabs, TabPayload};
use models::tab::BrowserTab;
use recorder::manager::RecorderManager;
use recorder::session::RecordingSession;

const APP_NAME: &str = "TabForge";
const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;
const DEFAULT_FPS: u32 = 60;

#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! Welcome to {}.",
        name,
        APP_NAME
    )
}

fn create_demo_tab() -> BrowserTab {
    BrowserTab {
        id: "tab_001".to_string(),
        title: "YouTube".to_string(),
        url: "https://youtube.com".to_string(),
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        fps: DEFAULT_FPS,
    }
}

fn create_demo_session() -> RecordingSession {
    RecordingSession {
        tab: create_demo_tab(),
        output_file:
            "youtube_1080p.mp4".to_string(),
        recording: false,
    }
}

fn bootstrap_session_manager() -> RecorderManager {
    let mut manager =
        RecorderManager::new();

    manager
        .sessions
        .push(
            create_demo_session()
        );

    manager
}

fn bootstrap_ipc_demo() {
    let payload = TabPayload {

        selected_tabs: vec![
            "youtube".to_string(),
            "research".to_string(),
            "docs".to_string(),
        ],

        timestamp: 123456789,
    };

    receive_tabs(payload);
}

fn print_banner() {
    println!();
    println!(
        "========== {} ==========",
        APP_NAME
    );
}

fn print_session_info(
    manager: &RecorderManager
) {

    print_banner();

    for (
        index,
        session
    ) in manager
        .sessions
        .iter()
        .enumerate()
    {

        println!(
            "[{}] {} ({})",
            index + 1,
            session.tab.title,
            session.tab.id
        );

        println!(
            "URL: {}",
            session.tab.url
        );

        println!(
            "Resolution: {}x{}",
            session.tab.width,
            session.tab.height
        );

        println!(
            "FPS: {}",
            session.tab.fps
        );

        println!(
            "Output: {}",
            session.output_file
        );

        println!(
            "Recording: {}",
            session.recording
        );

        println!(
            "---------------------"
        );
    }

    println!(
        "Active Sessions: {}",
        manager.sessions.len()
    );

    println!(
        "======================"
    );

    println!();
}

fn initialize_runtime() {

    println!(
        "[BOOT] Starting {}...",
        APP_NAME
    );

    println!(
        "[BOOT] Runtime initialized"
    );
}

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point
)]
pub fn run() {

    initialize_runtime();

    let manager =
        bootstrap_session_manager();

    print_session_info(
        &manager
    );

    bootstrap_ipc_demo();

    tauri::Builder::default()

        .plugin(
            tauri_plugin_opener::init()
        )

        .invoke_handler(
            tauri::generate_handler![
                greet
            ]
        )

        .run(
            tauri::generate_context!()
        )

        .expect(
            "Failed to start TabForge"
        );
}