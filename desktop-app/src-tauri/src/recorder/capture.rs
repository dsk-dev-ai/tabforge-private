use crate::recorder::session::
    RecordingSession;


#[allow(dead_code)]

pub fn start_capture(

    session:
    &RecordingSession

){

    println!();

    println!(
        "===== CAPTURE SESSION ====="
    );

    println!(
        "Tab: {}",
        session.tab.title
    );

    println!(
        "Tab ID: {}",
        session.tab.id
    );

    println!(
        "Browser: {}",
        session.tab.browser
    );

    println!(
        "URL: {}",
        session.tab.url
    );

    println!(
        "Resolution: {}",

        session
        .tab
        .resolution()

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
        "Audio Bitrate: {}",
        session.audio_bitrate
    );

    println!(
        "Video Bitrate: {}",
        session.video_bitrate
    );

    println!(
        "Worker: {:?}",
        session.worker_id
    );

    println!(
        "Stream Connected: {}",
        session.stream_connected
    );

    println!(
        "Output: {}",
        session.output_file
    );

    println!(
        "[CAPTURE] Worker attached"
    );

    println!(
        "[CAPTURE] FFmpeg connected"
    );

    println!(
        "[CAPTURE] Routing active"
    );

    println!(
        "==========================="
    );

    println!();

}