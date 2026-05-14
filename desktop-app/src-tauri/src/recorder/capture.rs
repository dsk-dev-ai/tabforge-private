use crate::recorder::session::RecordingSession;

pub fn start_capture(
    session: &RecordingSession
) {

    println!(
        "[RECORDER] Capture started"
    );

    println!(
        "Tab: {}",
        session.tab.title
    );

    println!(
        "Output: {}",
        session.output_file
    );

    println!(
        "Audio Enabled: {}",
        session.audio_enabled
    );

    println!(
        "Video Enabled: {}",
        session.video_enabled
    );
}