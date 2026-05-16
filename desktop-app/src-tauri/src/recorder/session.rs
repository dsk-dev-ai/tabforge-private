use crate::models::tab::BrowserTab;

#[derive(
    Debug,
    Clone
)]

pub struct RecordingSession{

    pub tab:BrowserTab,

    pub output_file:String,

    pub recording:bool,

    pub fps:u32,

    pub audio_enabled:bool,

    pub video_enabled:bool,

    pub separate_audio:bool,

    pub hardware_encoding:bool,

    pub audio_bitrate:u32,

    pub video_bitrate:u32,

    pub worker_id:Option<u32>,

    pub stream_connected:bool
}


impl RecordingSession{


    #[allow(dead_code)]

    pub fn start(
        &mut self
    ){

        self.recording=true;

        println!(
            "[SESSION START] {}",
            self.tab.title
        );
    }



    #[allow(dead_code)]

    pub fn stop(
        &mut self
    ){

        self.recording=false;

        println!(
            "[SESSION STOP] {}",
            self.tab.title
        );
    }



    #[allow(dead_code)]

    pub fn attach_worker(
        &mut self,
        worker:u32
    ){

        self.worker_id=
            Some(worker);

        println!(
            "[SESSION] Worker {} attached",
            worker
        );

    }



    #[allow(dead_code)]

    pub fn connect_stream(
        &mut self
    ){

        self.stream_connected=true;

        println!(
            "[SESSION] Stream linked"
        );

    }



    #[allow(dead_code)]

    pub fn summary(
        &self
    ){

        println!();

        println!(
            "===== SESSION ====="
        );

        println!(
            "Tab: {}",
            self.tab.title
        );

        println!(
            "FPS: {}",
            self.fps
        );

        println!(
            "Audio: {}",
            self.audio_enabled
        );

        println!(
            "Video: {}",
            self.video_enabled
        );

        println!(
            "Separate Audio: {}",
            self.separate_audio
        );

        println!(
            "Hardware: {}",
            self.hardware_encoding
        );

        println!(
            "Audio Bitrate: {}",
            self.audio_bitrate
        );

        println!(
            "Video Bitrate: {}",
            self.video_bitrate
        );

        println!(
            "Worker: {:?}",
            self.worker_id
        );

        println!(
            "Stream: {}",
            self.stream_connected
        );

        println!(
            "Output: {}",
            self.output_file
        );

        println!(
            "===================="
        );

        println!();

    }

}