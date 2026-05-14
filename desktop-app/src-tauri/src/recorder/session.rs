use crate::models::tab::BrowserTab;

#[derive(Debug, Clone)]

pub struct RecordingSession {

    pub tab: BrowserTab,

    pub output_file: String,

    pub recording: bool,

    pub audio_enabled: bool,

    pub video_enabled: bool,

    pub hardware_encoding: bool,

    pub separate_audio: bool,

    pub fps: u32
}