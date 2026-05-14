use crate::models::tab::BrowserTab;

use crate::recorder::session::RecordingSession;

#[derive(Debug, Clone)]

pub struct TabData {

    pub id: String,

    pub title: String,

    pub url: String
}

#[derive(Debug, Clone)]

pub struct TabPayload {

    pub selected_tabs:
        Vec<TabData>,

    pub timestamp: u64
}

fn create_browser_tab(
    tab: &TabData
) -> BrowserTab {

    BrowserTab {

        id:
            tab.id.clone(),

        title:
            tab.title.clone(),

        url:
            tab.url.clone(),

        width:
            1920,

        height:
            1080,

        fps:
            60
    }
}

fn create_output_filename(
    title: &str
) -> String {

    let safe_name =

        title

        .to_lowercase()

        .replace(" ", "_")

        .replace("/", "_")

        .replace("\\", "_")

        .replace(":", "_");

    format!(
        "{}_1080p.mp4",
        safe_name
    )
}

fn create_recording_session(
    browser_tab: BrowserTab
) -> RecordingSession {

    let filename =

        create_output_filename(
            &browser_tab.title
        );

    RecordingSession {

        tab:
            browser_tab,

        output_file:
            filename,

        recording:
            false,

        audio_enabled:
            true,

        video_enabled:
            true,

        hardware_encoding:
            true,

        separate_audio:
            true,

        fps:
            60
    }
}

pub fn receive_tabs(
    payload: TabPayload
) -> Vec<RecordingSession> {

    println!();

    println!(
        "===== EXTENSION PAYLOAD ====="
    );

    let mut sessions =
        Vec::new();

    for tab in
        payload.selected_tabs
    {

        println!(
            "[{}]",
            tab.id
        );

        println!(
            "Title: {}",
            tab.title
        );

        println!(
            "URL: {}",
            tab.url
        );

        let browser_tab =
            create_browser_tab(
                &tab
            );

        let session =
            create_recording_session(
                browser_tab
            );

        println!(
            "Session created"
        );

        println!(
            "----------------"
        );

        sessions.push(
            session
        );
    }

    println!(
        "Timestamp: {}",
        payload.timestamp
    );

    println!(
        "============================="
    );

    println!();

    sessions
}