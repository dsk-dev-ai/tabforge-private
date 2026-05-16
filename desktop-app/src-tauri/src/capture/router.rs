#[derive(Debug)]

pub struct CaptureRouter {

    pub worker_per_tab: bool,

    pub parallel_streams: bool,

    pub audio_video_split: bool
}

pub fn initialize_router() -> CaptureRouter {

    let router=

    CaptureRouter{

        worker_per_tab:true,

        parallel_streams:true,

        audio_video_split:true
    };


    println!(
        "[ROUTER] Stream routing ready"
    );

    println!(
        "[ROUTER] One worker per tab"
    );

    println!(
        "[ROUTER] Parallel routing: {}",
        router.parallel_streams
    );

    println!(
        "[ROUTER] Audio/video split: {}",
        router.audio_video_split
    );

    router
}


#[allow(dead_code)]
pub fn route_stream(
    tab_id:&str
){

    println!(
        "[ROUTER] Routing {}",
        tab_id
    );

}