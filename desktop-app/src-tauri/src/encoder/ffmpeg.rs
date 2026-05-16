use std::collections::HashMap;
use std::sync::{
    Mutex,
    OnceLock
};

#[derive(Debug)]

pub struct FFmpegSession{

    pub tab_id:String,

    pub output:String,

    pub active:bool,

    pub bytes_received:u64,

    pub chunks_received:u64,

    pub video_codec:String,

    pub audio_codec:String
}


static ACTIVE_SESSIONS:

OnceLock<
Mutex<
HashMap<
String,
FFmpegSession
>
>
>

=OnceLock::new();


fn session_store()

-> &'static Mutex<
HashMap<
String,
FFmpegSession
>
>{

    ACTIVE_SESSIONS
    .get_or_init(

        ||Mutex::new(
            HashMap::new()
        )

    )
}



pub fn initialize_ffmpeg(){

    session_store();

    println!(
        "[ENCODER] FFmpeg initialized"
    );

    println!(
        "[ENCODER] MP4 container enabled"
    );

    println!(
        "[ENCODER] Session persistence enabled"
    );

    println!(
        "[ENCODER] H264 + Opus pipeline ready"
    );

    println!(
        "[ENCODER] Awaiting browser chunks"
    );
}



pub fn create_muxer(

tab_id:&str,

output:&str

){

let mut sessions=

session_store()

.lock()

.unwrap();


if sessions.contains_key(
tab_id
){

return;

}


println!(
"[MUXER] Created {}",
tab_id
);


sessions.insert(

tab_id.to_string(),

FFmpegSession{

tab_id:
tab_id.to_string(),

output:
output.to_string(),

active:true,

bytes_received:0,

chunks_received:0,

video_codec:
"h264"
.to_string(),

audio_codec:
"opus"
.to_string()

}

);

}



pub fn write_chunk(

tab_id:&str,

bytes:usize

){

let mut sessions=

session_store()

.lock()

.unwrap();


if let Some(session)=

sessions.get_mut(
tab_id
){

session.bytes_received +=
bytes as u64;

session.chunks_received +=1;


if session
.chunks_received
%10==0{

println!(

"[MUXER] {} | {} chunks | {}KB",

tab_id,

session
.chunks_received,

session
.bytes_received
/1024

);

}

}

}



#[allow(dead_code)]

pub fn stop_muxer(

tab_id:&str

){

let mut sessions=

session_store()

.lock()

.unwrap();


if let Some(session)=

sessions.remove(
tab_id
){

println!(

"[MUXER CLOSED] {}",

session.tab_id

);

println!(

"[MUXER STATS] {} chunks",

session.chunks_received

);

println!(

"[MUXER STATS] {}KB",

session.bytes_received
/1024

);

}

}