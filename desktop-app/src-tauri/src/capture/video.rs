#[derive(Debug)]

pub struct VideoEngine {

    pub width:u32,

    pub height:u32,

    pub fps:u32,

    pub codec:String,

    pub hardware_acceleration:bool
}

pub fn initialize_video()->VideoEngine{

    let engine=

    VideoEngine{

        width:1920,

        height:1080,

        fps:60,

        codec:
        "h264".to_string(),

        hardware_acceleration:true
    };


    println!(
        "[VIDEO] Tab capture ready"
    );

    println!(
        "[VIDEO] Resolution: {}x{}",
        engine.width,
        engine.height
    );

    println!(
        "[VIDEO] FPS: {}",
        engine.fps
    );

    println!(
        "[VIDEO] Codec: {}",
        engine.codec
    );

    println!(
        "[VIDEO] Hardware: {}",
        engine.hardware_acceleration
    );

    engine
}


#[allow(dead_code)]
pub fn attach_video_stream(
    tab_id:&str
){

    println!(
        "[VIDEO] Stream attached -> {}",
        tab_id
    );

}