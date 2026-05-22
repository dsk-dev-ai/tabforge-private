use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{BufWriter, Write},
    path::Path,
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};

// ==========================================
// GLOBAL METRICS
// ==========================================

#[derive(Debug, Default)]

pub struct EncoderMetrics {

    pub sessions_started:u64,

    pub sessions_closed:u64,

    pub total_chunks:u64,

    pub total_bytes:u64,

    pub write_errors:u64
}


// ==========================================
// SESSION OWNER
// ==========================================

#[derive(Debug)]

pub struct SessionMuxer{

    pub id:String,

    pub path:String,

    pub chunks:u64,

    pub bytes:usize,

    pub active:bool,

    pub started:u64,

    pub file:Option<BufWriter<File>>
}


// ==========================================
// GLOBAL STORE
// ==========================================

static MUXERS:
OnceLock<
Mutex<
HashMap<
String,
SessionMuxer
>
>
>=OnceLock::new();


static METRICS:
OnceLock<
Mutex<
EncoderMetrics
>
>=OnceLock::new();


fn muxers()

-> &'static Mutex<
HashMap<
String,
SessionMuxer
>
>{

MUXERS.get_or_init(

|| Mutex::new(
HashMap::new()
)

)

}


fn metrics()

-> &'static Mutex<
EncoderMetrics
>{

METRICS.get_or_init(

|| Mutex::new(
EncoderMetrics::default()
)

)

}



// ==========================================
// INIT
// ==========================================

pub fn initialize_ffmpeg(){

muxers();

metrics();

create_dir_all(
"recordings"
).ok();


println!(
"[FFMPEG] initialized"
);

println!(
"[FFMPEG] session owner active"
);

println!(
"[FFMPEG] mux runtime online"
);

println!(
"[FFMPEG] recording dir ready"
);

}



// ==========================================
// START
// ==========================================

pub fn start_session(

session_id:&str,

output:&str

){

let mut store=

match muxers().lock(){

Ok(v)=>v,

Err(_)=>{

println!(
"[MUX LOCK]"
);

return;

}

};


if store.contains_key(
session_id
){

return;

}


if let Some(parent)=

Path::new(output)
.parent()

{

create_dir_all(
parent
).ok();

}


let file=

match File::create(
output
){

Ok(v)=>

Some(
BufWriter::new(v)
),

Err(error)=>{

println!(
"[FILE ERROR] {}",
error
);

None

}

};


store.insert(

session_id.to_string(),

SessionMuxer{

id:
session_id.to_string(),

path:
output.to_string(),

chunks:0,

bytes:0,

active:true,

started:

SystemTime::now()

.duration_since(
UNIX_EPOCH
)

.unwrap()

.as_secs(),

file

}

);


if let Ok(mut m)=
metrics().lock(){

m.sessions_started+=1;

}


println!(
"[SESSION OWNER] {}",
session_id
);

println!(
"[OUTPUT] {}",
output
);

}



// ==========================================
// APPEND
// ==========================================

pub fn append_chunk(

session_id:&str,

bytes:&[u8]

){

let mut store=

match muxers().lock(){

Ok(v)=>v,

Err(_)=>return

};


if let Some(session)=

store.get_mut(
session_id
){

if !session.active{

return;

}


session.chunks+=1;

session.bytes+=
bytes.len();


if let Some(file)=
session.file.as_mut(){

if let Err(error)=

file.write_all(
bytes
){

println!(
"[WRITE ERROR] {}",
error
);


if let Ok(mut m)=
metrics().lock(){

m.write_errors+=1;

}

}

}


if let Ok(mut m)=
metrics().lock(){

m.total_chunks+=1;

m.total_bytes+=

bytes.len() as u64;

}


if session.chunks%10==0{

println!(

"[MUX {}] chunks={} bytes={}KB",

session_id,

session.chunks,

session.bytes/1024

);

}

}

}



// ==========================================
// CLOSE
// ==========================================

pub fn close_session(

session_id:&str

){

let mut store=

match muxers().lock(){

Ok(v)=>v,

Err(_)=>return

};


if let Some(mut session)=

store.remove(
session_id
){

session.active=false;


if let Some(writer)=
session.file.as_mut(){

let _=
writer.flush();

}


let duration=

SystemTime::now()

.duration_since(
UNIX_EPOCH
)

.unwrap()

.as_secs()

-

session.started;


if let Ok(mut m)=
metrics().lock(){

m.sessions_closed+=1;

}


println!(
"[MUX CLOSED] {}",
session_id
);

println!(
"[DURATION] {}s",
duration
);

println!(
"[CHUNKS] {}",
session.chunks
);

println!(
"[BYTES] {}KB",
session.bytes/1024
);

}

}



// ==========================================
// DEBUG
// ==========================================

pub fn session_stats(){

let store=

match muxers().lock(){

Ok(v)=>v,

Err(_)=>return

};


println!();

println!(
"========== MUX =========="
);


for(_,session)

in store.iter(){

println!(
"session {}",
session.id
);

println!(
"chunks {}",
session.chunks
);

println!(
"bytes {}KB",
session.bytes/1024
);

println!(
"active {}",
session.active
);

println!(
"----------------"
);

}


println!(
"========================="
);

}