mod recorder;
mod encoder;
mod ipc;
mod workers;
mod models;

use ipc::bridge::{
    receive_tabs,
    TabPayload
};

use models::tab::BrowserTab;
use recorder::manager::RecorderManager;
use recorder::session::RecordingSession;

#[tauri::command]
fn greet(name: &str) -> String {
    format!(
        "Hello, {}! Welcome to TabForge.",
        name
    )
}

fn bootstrap_demo_session() -> RecorderManager {

    let youtube_tab = BrowserTab {
        id: "tab_001".to_string(),

        title: "YouTube".to_string(),

        url:
            "https://youtube.com"
            .to_string(),

        width: 1920,

        height: 1080,

        fps: 60,
    };

    let youtube_session =
        RecordingSession {

        tab: youtube_tab,

        output_file:
            "youtube_1080p.mp4"
            .to_string(),

        recording: false,
    };

    let mut manager =
        RecorderManager::new();

    manager
        .sessions
        .push(
            youtube_session
        );

    manager
}

fn bootstrap_ipc_demo() {

    let payload =
        TabPayload {

        selected_tabs:
            vec![

            "youtube"
                .to_string(),

            "research"
                .to_string(),

            "docs"
                .to_string()
        ],

        timestamp: 123456789,
    };

    receive_tabs(
        payload
    );
}

fn print_session_info(
    manager:
    &RecorderManager
) {

    println!();

    println!(
        "========== TABFORGE =========="
    );

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

            session
                .tab
                .title,

            session
                .tab
                .id
        );

        println!(
            "URL: {}",

            session
                .tab
                .url
        );

        println!(
            "Resolution: {}x{}",

            session
                .tab
                .width,

            session
                .tab
                .height
        );

        println!(
            "FPS: {}",

            session
                .tab
                .fps
        );

        println!(
            "Output: {}",

            session
                .output_file
        );

        println!(
            "Recording: {}",

            session
                .recording
        );

        println!(
            "--------------------"
        );
    }

    println!(
        "Active Sessions: {}",

        manager
            .sessions
            .len()
    );

    println!(
        "===================="
    );

    println!();
}

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point
)]
pub fn run() {

    let manager =
        bootstrap_demo_session();

    print_session_info(
        &manager
    );

    bootstrap_ipc_demo();

    tauri::Builder::default()

        .plugin(
            tauri_plugin_opener
            ::init()
        )

        .invoke_handler(
            tauri::generate_handler![
                greet
            ]
        )

        .run(
            tauri
            ::generate_context!()
        )

        .expect(
            "error while running app"
        );
}