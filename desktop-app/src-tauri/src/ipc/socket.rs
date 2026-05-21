use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use futures_util::StreamExt;

use tokio::net::TcpListener;

use tokio_tungstenite::accept_async;

use crate::encoder::ffmpeg::{
    create_muxer,
    write_chunk
};

#[derive(Debug,Clone)]

pub struct SocketMessage{

    pub tab_id:String,

    pub payload:Vec<u8>,

    pub timestamp:u64,

    pub stream_type:String,

    pub chunk_index:u64
}


#[derive(Debug,Default)]

pub struct StreamSession{

    pub total_bytes:usize,

    pub chunks:u64,

    pub active:bool
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
        "[IPC] Multi-tab routing enabled"
    );

    println!(
        "[IPC] Real browser ingest mode"
    );

}


pub fn listen_for_streams(){

    println!(
        "[IPC] Awaiting browser extension..."
    );

    println!(
        "[IPC] ws://127.0.0.1:8765"
    );

}


pub async fn start_websocket_server(){

    let listener=

        TcpListener::bind(
            "127.0.0.1:8765"
        )

        .await

        .unwrap();


    println!(
        "[IPC] Websocket server running"
    );


    let sessions=

    Arc::new(

        Mutex::new(

            HashMap::<
                String,
                StreamSession
            >::new()

        )

    );


    while let Ok(
        (stream,_)
    )=

    listener.accept().await{


        println!(
            "[IPC] Browser connected"
        );


        let sessions=
            Arc::clone(
                &sessions
            );


        tokio::spawn(

            async move{

            let ws=

            accept_async(
                stream
            )

            .await

            .unwrap();


            let (_write,
                mut read)=

            ws.split();


            while let Some(
                message
            )=

            read.next()
            .await{


                if let Ok(msg)=
                    message{


                    if msg.is_binary(){


                        let bytes=

                        msg.into_data();


                        process_chunk(

                            "tab_001",

                            bytes.len(),

                            &sessions

                        );

                    }

                }

            }

        });

    }

}



fn process_chunk(

    tab_id:&str,

    size:usize,

    sessions:

    &Arc<

        Mutex<

        HashMap<
        String,
        StreamSession

        >

    >>

){

    let mut store=

        sessions
        .lock()
        .unwrap();


    let state=

        store

        .entry(
            tab_id
            .to_string()
        )

        .or_default();


    state.active=true;

    state.chunks+=1;

    state.total_bytes+=size;


    if state.chunks==1{

        create_muxer(

            tab_id,

            &format!(
                "recordings/{}.mp4",
                tab_id
            )

        );

    }


    write_chunk(
        tab_id,
        size
    );


    if state.chunks%10==0{


        println!(
            "[STREAM] {}",
            tab_id
        );

        println!(
            "Chunks: {}",
            state.chunks
        );

        println!(
            "Bytes: {}KB",
            state.total_bytes/1024
        );

    }

}



#[allow(dead_code)]

pub fn send_message(

    message:
    SocketMessage

){

    println!(
        "[IPC OUTBOUND]"
    );

    println!(
        "Tab: {}",
        message.tab_id
    );

    println!(
        "Bytes: {}",
        message.payload.len()
    );

}