#[derive(Debug)]
pub struct AudioEngine {

    pub per_tab_isolation: bool,

    pub mixed_audio_disabled: bool,

    pub separate_tracks: bool,

    pub audio_codec: String,

    pub bitrate: u32
}

pub fn initialize_audio() -> AudioEngine {

    let engine = AudioEngine {

        per_tab_isolation: true,

        mixed_audio_disabled: true,

        separate_tracks: true,

        audio_codec:
            "opus".to_string(),

        bitrate: 320
    };

    println!(
        "[AUDIO] Audio engine online"
    );

    println!(
        "[AUDIO] Per-tab isolation: {}",
        engine.per_tab_isolation
    );

    println!(
        "[AUDIO] Desktop mix disabled: {}",
        engine.mixed_audio_disabled
    );

    println!(
        "[AUDIO] Separate tracks: {}",
        engine.separate_tracks
    );

    println!(
        "[AUDIO] Codec: {}",
        engine.audio_codec
    );

    println!(
        "[AUDIO] Bitrate: {} kbps",
        engine.bitrate
    );

    engine
}

#[allow(dead_code)]
pub fn attach_audio_stream(
    tab_id:&str
){

    println!(
        "[AUDIO] Attached stream -> {}",
        tab_id
    );

}

#[allow(dead_code)]
pub fn detach_audio_stream(
    tab_id:&str
){

    println!(
        "[AUDIO] Detached stream -> {}",
        tab_id
    );

}