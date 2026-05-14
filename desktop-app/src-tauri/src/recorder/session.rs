use crate::models::tab::BrowserTab;

pub struct RecordingSession {
    pub tab: BrowserTab,
    pub output_file: String,
    pub recording: bool,
}