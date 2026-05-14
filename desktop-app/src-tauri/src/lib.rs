mod capture;
mod runtime;

mod recorder;
mod encoder;
mod ipc;
mod workers;
mod models;

use runtime::bootstrap::{
    initialize_runtime_services
};

use ipc::bridge::{
    receive_tabs,
    TabData,
    TabPayload
};

use models::tab::BrowserTab;

use recorder::manager::RecorderManager;

use recorder::session::RecordingSession;

const APP_NAME: &str =
    "TabForge";

const VERSION: &str =
    "0.1.0";

const DEFAULT_WIDTH: u32 =
    1920;

const DEFAULT_HEIGHT: u32 =
    1080;

const DEFAULT_FPS: u32 =
    60;

const DEFAULT_AUDIO_BITRATE: u32 =
    320;

const DEFAULT_VIDEO_BITRATE: u32 =
    8000;

#[tauri::command]
fn greet(
    name: &str
) -> String {

    format!(
        "Hello {}, welcome to {} {}",
        name,
        APP_NAME,
        VERSION
    )
}

fn create_browser_tab(
    id: &str,
    title: &str,
    url: &str
) -> BrowserTab {

    BrowserTab {

        id:
            id.to_string(),

        title:
            title.to_string(),

        url:
            url.to_string(),

        width:
            DEFAULT_WIDTH,

        height:
            DEFAULT_HEIGHT,

        fps:
            DEFAULT_FPS
    }
}

fn sanitize_filename(
    value: &str
) -> String {

    value

        .to_lowercase()

        .replace(" ", "_")

        .replace("/", "_")

        .replace("\\", "_")

        .replace(":", "_")
}

fn create_output_filename(
    tab: &BrowserTab
) -> String {

    let safe_name =
        sanitize_filename(
            &tab.title
        );

    format!(
        "{}_{}p.mp4",
        safe_name,
        tab.height
    )
}

fn create_recording_session(
    tab: BrowserTab
) -> RecordingSession {

    RecordingSession {

        output_file:
            create_output_filename(
                &tab
            ),

        fps:
            tab.fps,

        tab,

        recording:
            false,

        audio_enabled:
            true,

        video_enabled:
            true,

        hardware_encoding:
            true,

        separate_audio:
            true
    }
}

fn build_demo_manager()
-> RecorderManager {

    let mut manager =
        RecorderManager::new();

    let tabs = vec![

        create_browser_tab(
            "tab_001",
            "YouTube",
            "https://youtube.com"
        ),

        create_browser_tab(
            "tab_002",
            "Research",
            "https://openai.com"
        ),

        create_browser_tab(
            "tab_003",
            "Docs",
            "https://docs.rs"
        )
    ];

    for tab in tabs {

        manager
            .sessions
            .push(
                create_recording_session(
                    tab
                )
            );
    }

    manager
}

fn simulate_extension_payload() {

    let payload =

        TabPayload {

        selected_tabs:

            vec![

                TabData {

                    id:
                        "tab_001"
                        .to_string(),

                    title:
                        "YouTube"
                        .to_string(),

                    url:
                        "https://youtube.com"
                        .to_string()
                },

                TabData {

                    id:
                        "tab_002"
                        .to_string(),

                    title:
                        "Research"
                        .to_string(),

                    url:
                        "https://openai.com"
                        .to_string()
                },

                TabData {

                    id:
                        "tab_003"
                        .to_string(),

                    title:
                        "Docs"
                        .to_string(),

                    url:
                        "https://docs.rs"
                        .to_string()
                }
            ],

        timestamp:
            123456789
    };

    let sessions =
        receive_tabs(
            payload
        );

    print_dynamic_sessions(
        sessions
    );
}

fn print_dynamic_sessions(
    sessions:
    Vec<RecordingSession>
) {

    println!();

    println!(
        "===== SESSION BUILDER ====="
    );

    for session in sessions {

        println!(
            "{} -> {}",
            session.tab.title,
            session.output_file
        );

        println!(
            "Audio Enabled: {}",
            session.audio_enabled
        );

        println!(
            "Separate Audio: {}",
            session.separate_audio
        );

        println!(
            "Hardware Encoding: {}",
            session.hardware_encoding
        );

        println!(
            "---------------------------"
        );
    }

    println!();
}

fn print_dashboard(
    manager: &RecorderManager
) {

    println!();

    println!(
        "========== {} {} ==========",
        APP_NAME,
        VERSION
    );

    for (
        index,
        session
    )

    in manager
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
            session.fps
        );

        println!(
            "Audio Enabled: {}",
            session.audio_enabled
        );

        println!(
            "Video Enabled: {}",
            session.video_enabled
        );

        println!(
            "Separate Audio: {}",
            session.separate_audio
        );

        println!(
            "Hardware Encoding: {}",
            session.hardware_encoding
        );

        println!(
            "Audio Bitrate: {} kbps",
            DEFAULT_AUDIO_BITRATE
        );

        println!(
            "Video Bitrate: {} kbps",
            DEFAULT_VIDEO_BITRATE
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
            "---------------------------"
        );
    }

    println!(
        "Active Sessions: {}",
        manager.sessions.len()
    );

    println!(
        "============================"
    );

    println!();
}

fn initialize_runtime() {

    println!(
        "[BOOT] Starting {} {}",
        APP_NAME,
        VERSION
    );

    initialize_runtime_services();

    println!(
        "[BOOT] Runtime initialized"
    );

    println!(
        "[BOOT] Multi-tab session engine online"
    );

    println!(
        "[BOOT] Per-tab audio isolation enabled"
    );

    println!(
        "[BOOT] FFmpeg pipeline ready"
    );

    println!(
        "[BOOT] Hardware acceleration enabled"
    );
}

#[cfg_attr(
    mobile,
    tauri::mobile_entry_point
)]

pub fn run() {

    initialize_runtime();

    let manager =
        build_demo_manager();

    print_dashboard(
        &manager
    );

    simulate_extension_payload();

    tauri::Builder
        ::default()

        .plugin(
            tauri_plugin_opener
                ::init()
        )

        .invoke_handler(

            tauri
            ::generate_handler![

                greet
            ]
        )

        .run(
            tauri
                ::generate_context!()
        )

        .expect(
            "TabForge startup failed"
        );
}