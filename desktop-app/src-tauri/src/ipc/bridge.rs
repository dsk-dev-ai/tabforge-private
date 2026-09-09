use crate::models::tab::BrowserTab;

use crate::recorder::session::RecordingSession;

#[derive(Debug, Clone)]

pub struct TabData {
    pub id: String,

    pub title: String,

    pub url: String,
}

#[derive(Debug, Clone)]

pub struct TabPayload {
    pub selected_tabs: Vec<TabData>,

    pub timestamp: u64,
}

fn create_browser_tab(tab: &TabData) -> BrowserTab {
    BrowserTab {
        id: tab.id.clone(),

        title: tab.title.clone(),

        url: tab.url.clone(),

        width: 1920,

        height: 1080,

        fps: 60,
    }
}

fn create_output_filename(title: &str) -> String {
    let safe_name = title
        .to_lowercase()
        .replace(" ", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_");

    format!("{}_1080p.mp4", safe_name)
}

fn create_recording_session(browser_tab: BrowserTab) -> RecordingSession {
    let filename = create_output_filename(&browser_tab.title);

    RecordingSession {
        tab: browser_tab,

        output_file: filename,

        recording: false,

        audio_enabled: true,

        video_enabled: true,

        hardware_encoding: true,

        separate_audio: true,

        fps: 60,
    }
}

pub fn receive_tabs(payload: TabPayload) -> Vec<RecordingSession> {
    println!();

    println!("===== EXTENSION PAYLOAD =====");

    let mut sessions = Vec::new();

    for tab in payload.selected_tabs {
        println!("[{}]", tab.id);

        println!("Title: {}", tab.title);

        println!("URL: {}", tab.url);

        let browser_tab = create_browser_tab(&tab);

        let session = create_recording_session(browser_tab);

        println!("Session created");

        println!("----------------");

        sessions.push(session);
    }

    println!("Timestamp: {}", payload.timestamp);

    println!("=============================");

    println!();

    sessions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_filename_sanitizes_whitespace_and_separators() {
        assert_eq!(
            create_output_filename("My Great Tab"),
            "my_great_tab_1080p.mp4"
        );
        assert_eq!(
            create_output_filename("a/b\\c:d"),
            "a_b_c_d_1080p.mp4"
        );
    }

    #[test]
    fn output_filename_lowercases_title() {
        assert_eq!(
            create_output_filename("CamelCase Title"),
            "camelcase_title_1080p.mp4"
        );
    }

    #[test]
    fn create_browser_tab_defaults_resolution_and_fps() {
        let tab = create_browser_tab(&TabData {
            id: "tab-1".into(),
            title: "Docs".into(),
            url: "https://example.com".into(),
        });
        assert_eq!(tab.id, "tab-1");
        assert_eq!(tab.width, 1920);
        assert_eq!(tab.height, 1080);
        assert_eq!(tab.fps, 60);
    }

    #[test]
    fn receive_tabs_creates_one_session_per_selected_tab() {
        let payload = TabPayload {
            selected_tabs: vec![
                TabData {
                    id: "a".into(),
                    title: "Alpha Page".into(),
                    url: "https://a.test".into(),
                },
                TabData {
                    id: "b".into(),
                    title: "Beta Page".into(),
                    url: "https://b.test".into(),
                },
            ],
            timestamp: 1234,
        };
        let sessions = receive_tabs(payload);
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].output_file, "alpha_page_1080p.mp4");
        assert_eq!(sessions[1].output_file, "beta_page_1080p.mp4");
        assert!(sessions[0].recording == false);
    }
}
