use std::collections::HashMap;
use std::sync::{Arc,Mutex};
use std::thread;
use std::time::{
    Duration,
    SystemTime,
    UNIX_EPOCH
};

use crate::encoder::ffmpeg::{
    create_muxer,
    write_chunk
};

const SIMULATION_MODE:bool=true;

const CHUNK_SIZE:usize=4096;

const INTERVAL:u64=3;


#[derive(Debug,Clone)]

pub struct SocketMessage{

    pub tab_id:String,

    pub payload_size:usize,

    pub timestamp:u64,

    pub chunk_index:u64,

    pub stream_type:String,

    pub stream_active:bool
}


#[derive(Debug,Default)]

pub struct StreamStats{

    pub total_bytes:usize,

    pub total_chunks:u64,

    pub started:bool
}


pub fn initialize_socket_runtime(){

    println!(
        "[IPC] Socket initialized"
    );

    println!(
        "[IPC] WebSocket online"
    );

    println!(
        "[IPC] Binary mode enabled"
    );

    println!(
        "[IPC] Simulation: {}",
        SIMULATION_MODE
    );
}



pub fn listen_for_streams(){

    println!(
        "[IPC] Listening..."
    );

    if !SIMULATION_MODE{

        println!(
            "[IPC] Waiting extension connection"
        );

        return;
    }


    let stats=

    Arc::new(

        Mutex::new(

            HashMap::<
                String,
                StreamStats
            >::new()

        )
    );


    thread::spawn({

        let stream_stats=
            Arc::clone(
                &stats
            );

        move||{

            let tabs=vec![

                "tab_001",

                "tab_002",

                "tab_003"
            ];

            let mut chunk=0;


            loop{

                thread::sleep(

                    Duration::from_secs(
                        INTERVAL
                    )
                );

                chunk+=1;

                for tab in &tabs{

                    process_stream_chunk(

                        SocketMessage{

                            tab_id:
                            tab.to_string(),

                            payload_size:
                            CHUNK_SIZE,

                            timestamp:
                            timestamp(),

                            chunk_index:
                            chunk,

                            stream_type:
                            "video/webm"
                            .to_string(),

                            stream_active:
                            true

                        },

                        &stream_stats
                    );
                }
            }
        }
    });
}



fn process_stream_chunk(

message:SocketMessage,

stats:
&Arc<
Mutex<
HashMap<
String,
StreamStats
>
>
>

){

let mut map=
stats.lock().unwrap();


let session=

map.entry(
message.tab_id.clone()
)

.or_default();


session.total_bytes+=
message.payload_size;

session.total_chunks+=1;


if !session.started{

session.started=true;

create_muxer(

&message.tab_id,

&format!(
"recordings/{}.mp4",
message.tab_id
)

);

}


write_chunk(

&message.tab_id,

message.payload_size

);


if message.chunk_index %10==0{

println!(

"[STREAM] {} | chunks={} | total={}KB",

message.tab_id,

session.total_chunks,

session.total_bytes/1024

);

}

}



fn timestamp()->u64{

SystemTime::now()

.duration_since(
UNIX_EPOCH
)

.unwrap()

.as_secs()

}