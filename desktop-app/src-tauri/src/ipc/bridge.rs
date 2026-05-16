use crate::models::tab::BrowserTab;
use crate::recorder::session::RecordingSession;

const WIDTH:u32=1920;
const HEIGHT:u32=1080;
const FPS:u32=60;

const AUDIO_BITRATE:u32=320;
const VIDEO_BITRATE:u32=8000;


#[derive(
    Debug,
    Clone
)]

pub struct TabData{

    pub id:String,

    pub title:String,

    pub url:String
}


#[derive(Debug)]

pub struct TabPayload{

    pub selected_tabs:
        Vec<TabData>,

    pub timestamp:u64
}



pub fn receive_tabs(
    payload:TabPayload
)->Vec<RecordingSession>{

    println!();

    println!(
        "===== EXTENSION PAYLOAD ====="
    );

    let mut sessions=
        Vec::new();


    for tab in payload.selected_tabs{

        print_tab(
            &tab
        );

        let session=
            create_session(
                &tab
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



fn create_session(

    tab:&TabData

)->RecordingSession{


    let browser_tab=

    BrowserTab{

        id:
            tab.id.clone(),

        title:
            tab.title.clone(),

        url:
            tab.url.clone(),

        width:
            WIDTH,

        height:
            HEIGHT,

        fps:
            FPS,

        browser:
            "Chrome"
            .to_string(),

        active:
            true,

        audio_available:
            true,

        stream_type:
            "video/webm"
            .to_string()
    };


    RecordingSession{

        tab:
            browser_tab,

        output_file:
            create_filename(
                &tab.title
            ),

        recording:
            false,

        fps:
            FPS,

        audio_enabled:
            true,

        video_enabled:
            true,

        separate_audio:
            true,

        hardware_encoding:
            true,

        audio_bitrate:
            AUDIO_BITRATE,

        video_bitrate:
            VIDEO_BITRATE,

        worker_id:
            None,

        stream_connected:
            false
    }

}



fn create_filename(
    title:&str
)->String{

    format!(
        "{}_1080p.mp4",

        title

        .to_lowercase()

        .replace(
            " ",
            "_"
        )

        .replace(
            "/",
            "_"
        )
    )

}



fn print_tab(
    tab:&TabData
){

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

}