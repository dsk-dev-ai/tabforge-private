use std::{
    collections::HashMap,
    sync::{Arc,Mutex},
    time::{SystemTime,UNIX_EPOCH}
};

use futures_util::StreamExt;

use tokio::net::{
    TcpListener,
    TcpStream
};

use tokio_tungstenite::accept_async;

use serde::Deserialize;

use crate::encoder::ffmpeg::{
    start_session,
    append_chunk,
    close_session
};

use crate::workers::pool::{
    assign_worker,
    release_worker
};


// ========================================
// STORE
// ========================================

type SessionStore=
Arc<
Mutex<
HashMap<
String,
StreamSession
>>
>;


// ========================================
// SESSION
// ========================================

#[derive(Debug)]

pub struct StreamSession{

    pub session_id:String,

    pub tab_id:String,

    pub worker:Option<u32>,

    pub chunks:u64,

    pub total_bytes:usize,

    pub active:bool,

    pub started:u64
}


// ========================================
// META
// ========================================

#[derive(
Debug,
Deserialize
)]

#[serde(rename_all="camelCase")]

struct MetaPacket{

    #[serde(rename="type")]
    packet_type:String,

    session_id:String,

    tab_id:String,

    chunk:u64,

    time:u64,

    size:usize
}


#[derive(
Debug,
Deserialize
)]

#[serde(rename_all="camelCase")]

struct SessionEnd{

    session_id:String
}



// ========================================
// INIT
// ========================================

pub fn initialize_socket_runtime(){

println!();

println!("========== IPC ==========");

println!("[IPC] websocket enabled");

println!("[IPC] native session ownership");

println!("[IPC] transport runtime");

println!("[IPC] worker routing");

println!("=========================");

println!();

}


pub async fn listen_for_streams(){

println!(
"[IPC] awaiting extension"
);

println!(
"[IPC] ws://127.0.0.1:8765"
);

}



// ========================================
// SERVER
// ========================================

pub async fn start_websocket_server(){

let listener=

match TcpListener::bind(
"127.0.0.1:8765"
)

.await{

Ok(v)=>v,

Err(error)=>{

println!(
"[BIND ERROR] {}",
error
);

return;

}

};


println!(
"[WS ONLINE]"
);


let sessions:
SessionStore=

Arc::new(

Mutex::new(
HashMap::new()
)

);


loop{

match listener.accept().await{

Ok((stream,_))=>{

println!(
"[BROWSER CONNECTED]"
);

let sessions=

Arc::clone(
&sessions
);


tokio::spawn(

async move{

handle_connection(
stream,
sessions
)

.await;

}

);

}

Err(error)=>{

println!(
"[CONNECT ERROR] {}",
error
);

}

}

}

}



// ========================================
// CONNECTION
// ========================================

async fn handle_connection(

stream:TcpStream,

sessions:SessionStore

){

let ws=

match accept_async(
stream
)

.await{

Ok(v)=>v,

Err(error)=>{

println!(
"[UPGRADE ERROR] {}",
error
);

return;

}

};


let(
_,
mut read
)=ws.split();


let mut active_session=

String::new();


while let Some(msg)=
read.next().await{

match msg{

Ok(message)=>{


if message.is_close(){

if !active_session.is_empty(){

shutdown_session(

&active_session,
&sessions

);

}

break;

}



if message.is_binary(){

let bytes=

message.into_data();


if active_session.is_empty(){

continue;

}


process_chunk(

&active_session,

bytes,

&sessions

);

continue;

}



if message.is_text(){

let text=

match message.into_text(){

Ok(v)=>v,

Err(_)=>continue

};


if text.contains(
"heartbeat"
){

continue;

}


if text.contains(
"session_end"
){

let packet:

Result<
SessionEnd,
_
>

=

serde_json
::from_str(
&text
);


if let Ok(p)=packet{

shutdown_session(

&p.session_id,
&sessions

);

}

continue;

}



let meta:

Result<
MetaPacket,
_
>

=

serde_json
::from_str(
&text
);


if let Ok(meta)=meta{

active_session=

meta.session_id
.clone();


create_session(

meta,

&sessions

);

}

}

}

Err(error)=>{

println!(
"[STREAM ERROR] {}",
error
);

break;

}

}

}

}



// ========================================
// CREATE
// ========================================

fn create_session(

meta:MetaPacket,

sessions:&SessionStore

){

let mut store=

sessions
.lock()
.unwrap();


if store.contains_key(
&meta.session_id
){

return;

}


let worker=

assign_worker(
&meta.session_id
);


start_session(

&meta.session_id,

&format!(

"recordings/{}.webm",

meta.session_id

)

);


store.insert(

meta.session_id.clone(),

StreamSession{

session_id:
meta.session_id.clone(),

tab_id:
meta.tab_id,

worker,

chunks:0,

total_bytes:0,

active:true,

started:

SystemTime::now()

.duration_since(
UNIX_EPOCH
)

.unwrap()

.as_secs()

}

);


println!(
"[SESSION START]"
);

}



// ========================================
// PROCESS
// ========================================

fn process_chunk(

session_id:&str,

bytes:Vec<u8>,

sessions:&SessionStore

){

let mut store=

sessions
.lock()
.unwrap();


if let Some(session)=

store.get_mut(
session_id
){

session.chunks+=1;

session.total_bytes+=

bytes.len();


append_chunk(

session_id,

&bytes

);


if session.chunks%10==0{

println!();

println!(
"[STREAM {}]",
session_id
);

println!(
"tab {}",
session.tab_id
);

println!(
"worker {:?}",
session.worker
);

println!(
"chunks {}",
session.chunks
);

println!(
"bytes {}KB",
session.total_bytes/1024
);

println!(
"----------------"
);

}

}

}



// ========================================
// SHUTDOWN
// ========================================

fn shutdown_session(

session_id:&str,

sessions:&SessionStore

){

let mut store=

sessions
.lock()
.unwrap();


store.remove(
session_id
);


drop(store);


release_worker(
session_id
);


close_session(
session_id
);


println!(
"[SESSION CLOSED] {}",
session_id
);

}