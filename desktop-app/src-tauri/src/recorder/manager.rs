use super::session::RecordingSession;

pub struct RecorderManager {
    pub sessions: Vec<RecordingSession>
}

impl RecorderManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new()
        }
    }
}