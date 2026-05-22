use std::{
    collections::HashMap,
    sync::{Mutex,OnceLock},
    time::{SystemTime,UNIX_EPOCH}
};


// ====================================
// WORKER
// ====================================

#[derive(
Debug,
Clone
)]

pub struct Worker{

    pub id:u32,

    pub active:bool,

    pub busy:bool,

    pub sessions:u64,

    pub current_session:Option<String>,

    pub started:u64,

    pub bytes_processed:u64,

    pub chunks_processed:u64
}



// ====================================
// METRICS
// ====================================

#[derive(
Debug,
Default
)]

pub struct PoolMetrics{

    pub assigned:u64,

    pub released:u64,

    pub bytes:u64,

    pub chunks:u64
}



// ====================================
// GLOBALS
// ====================================

static WORKERS:

OnceLock<
Mutex<
HashMap<
u32,
Worker
>>>

=OnceLock::new();


static METRICS:

OnceLock<
Mutex<
PoolMetrics
>>

=OnceLock::new();



fn pool()

-> &'static Mutex<HashMap<u32,Worker>>{

WORKERS.get_or_init(

||Mutex::new(
HashMap::new()
)

)

}


fn metrics()

-> &'static Mutex<PoolMetrics>{

METRICS.get_or_init(

||Mutex::new(
PoolMetrics::default()
)

)

}



// ====================================
// TIME
// ====================================

fn now()->u64{

SystemTime::now()

.duration_since(
UNIX_EPOCH
)

.unwrap()

.as_secs()

}



// ====================================
// INIT
// ====================================

pub fn initialize_workers(){

let mut workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>{

println!(
"[WORKER LOCK]"
);

return;

}

};


workers.clear();


for id in 1..=6{

workers.insert(

id,

Worker{

id,

active:true,

busy:false,

sessions:0,

current_session:None,

started:now(),

bytes_processed:0,

chunks_processed:0

}

);

}


metrics();


println!(
"[WORKERS INITIALIZED]"
);

println!(
"[POOL SIZE] {}",
workers.len()
);

}



// ====================================
// ASSIGN
// ====================================

pub fn assign_worker(

session_id:&str

)->Option<u32>{

let mut workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>return None

};


let selected=

workers

.values_mut()

.filter(

|w|

w.active
&&
!w.busy

)

.min_by_key(

|w|

w.sessions

);


if let Some(worker)=selected{

worker.busy=true;

worker.sessions+=1;

worker.current_session=

Some(

session_id
.to_string()

);


if let Ok(mut m)=

metrics().lock(){

m.assigned+=1;

}


println!(

"[ASSIGN] worker={} session={}",

worker.id,

session_id

);


return Some(

worker.id

);

}


println!(
"[NO WORKER]"
);

None

}



// ====================================
// RELEASE
// ====================================

pub fn release_worker(

session_id:&str

){

let mut workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>return

};


for worker in

workers.values_mut(){

if

worker

.current_session

.as_deref()

==

Some(
session_id
)

{

worker.busy=false;

worker.current_session=None;


if let Ok(mut m)=

metrics().lock(){

m.released+=1;

}


println!(

"[RELEASE] {}",

worker.id

);

return;

}

}

}



// ====================================
// TRACK
// ====================================

pub fn add_bytes(

worker_id:u32,

bytes:u64

){

let mut workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>return

};


if let Some(worker)=

workers.get_mut(
&worker_id
){

worker.bytes_processed+=bytes;

worker.chunks_processed+=1;


if let Ok(mut m)=

metrics().lock(){

m.bytes+=bytes;

m.chunks+=1;

}

}

}



// ====================================
// DEBUG
// ====================================

pub fn list_workers(){

let workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>return

};


println!();


for worker in

workers.values(){

println!(

"[WORKER {}]",

worker.id

);

println!(
"busy {}",
worker.busy
);

println!(
"sessions {}",
worker.sessions
);

println!(
"bytes {}KB",

worker.bytes_processed
/1024
);

println!(
"chunks {}",

worker.chunks_processed
);

println!(
"session {:?}",

worker.current_session
);

println!(
"------------"
);

}

}



// ====================================
// STATS
// ====================================

pub fn worker_stats(){

let workers=

match pool().lock(){

Ok(v)=>v,

Err(_)=>return

};


let total=

workers.len();


let busy=

workers

.values()

.filter(
|w|w.busy
)

.count();


drop(workers);


let m=

metrics()

.lock()

.unwrap();


println!();

println!(
"[POOL]"
);

println!(
"total={}",
total
);

println!(
"busy={}",
busy
);

println!(
"idle={}",
total-busy
);

println!(
"assigned={}",
m.assigned
);

println!(
"released={}",
m.released
);

println!(
"bytes={}KB",
m.bytes/1024
);

println!(
"chunks={}",
m.chunks
);

println!();

}