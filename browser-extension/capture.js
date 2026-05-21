// TabForge Live Browser Binary Stream Engine

const activeCaptures = new Map();

const sockets = new Map();

const WS_URL = "ws://127.0.0.1:8765";

const RECORD_INTERVAL = 1000;



async function initializeCaptureEngine(){

    console.log(
        "[TABFORGE] Capture engine online"
    );

    console.log(
        "[TABFORGE] Binary stream mode active"
    );

    console.log(
        "[TABFORGE] WebSocket:",
        WS_URL
    );
}



async function createSocket(
    tabId
){

    if(
        sockets.has(tabId)
    ){

        const existing=
            sockets.get(tabId);

        if(
            existing.readyState===1
        ){

            return existing;
        }
    }


    return new Promise(

        (resolve,reject)=>{

            const socket=
                new WebSocket(
                    WS_URL
                );

            socket.binaryType=
                "arraybuffer";


            socket.onopen=()=>{

                console.log(
                    `[SOCKET] Connected ${tabId}`
                );

                sockets.set(
                    tabId,
                    socket
                );

                resolve(
                    socket
                );

            };


            socket.onerror=(error)=>{

                console.error(

                    `[SOCKET ERROR] ${tabId}`,

                    error

                );

                reject(
                    error
                );

            };


            socket.onclose=()=>{

                console.log(
                    `[SOCKET CLOSED] ${tabId}`
                );

                sockets.delete(
                    tabId
                );

            };


            socket.onmessage=(msg)=>{

                console.log(

                    `[RUST→EXT] ${tabId}`,

                    msg.data

                );

            };

        }

    );

}



async function startTabCapture(
    tabId
){

try{

console.log(
`[CAPTURE] Starting ${tabId}`
);


const stream=

await chrome.tabCapture.capture({

audio:true,

video:true,

videoConstraints:{

mandatory:{

maxWidth:1920,

maxHeight:1080,

maxFrameRate:60

}

}

});


if(!stream){

throw new Error(
"Capture stream unavailable"
);

}


const socket=

await createSocket(
tabId
);


const recorder=

new MediaRecorder(

stream,

{

mimeType:
"video/webm;codecs=vp9,opus",

videoBitsPerSecond:
8000000,

audioBitsPerSecond:
320000

}

);



recorder.onstart=()=>{

console.log(
`[RECORDER] Active ${tabId}`
);

};



recorder.onerror=(e)=>{

console.error(

`[RECORDER ERROR] ${tabId}`,

e

);

};



recorder.ondataavailable=

async(event)=>{

if(

!event.data ||

event.data.size===0

){

return;

}


if(

socket.readyState!==1

){

console.warn(

`[SOCKET LOST] ${tabId}`

);

return;

}


try{

const buffer=

await event
.data
.arrayBuffer();


socket.send(

JSON.stringify({

tabId:

tabId,

timestamp:

Date.now(),

mime:

"video/webm",

size:

buffer.byteLength

})

);


socket.send(
buffer
);


console.log(

`[SEND] ${tabId}`,

`${buffer.byteLength} bytes`

);

}
catch(e){

console.error(

`[STREAM ERROR] ${tabId}`,

e

);

}

};



recorder.onstop=()=>{

console.log(

`[RECORDER] Stopped ${tabId}`

);

};



recorder.start(
RECORD_INTERVAL
);


activeCaptures.set(

tabId,

{

stream,

recorder,

socket,

started:

Date.now()

}

);


console.log(
`[LIVE] ${tabId}`
);

}
catch(error){

console.error(

`[CAPTURE FAILED] ${tabId}`,

error

);

}

}



function stopTabCapture(
tabId
){

const capture=

activeCaptures.get(
tabId
);


if(!capture){

console.warn(

`[STOP] No session ${tabId}`

);

return;

}


capture.recorder.stop();


capture.stream

.getTracks()

.forEach(

track=>

track.stop()

);


if(

capture.socket &&

capture.socket.readyState===1

){

capture.socket.send(

JSON.stringify({

tabId,

closed:true

})

);

capture.socket.close();

}


activeCaptures.delete(
tabId
);


console.log(

`[CAPTURE RELEASED] ${tabId}`

);

}



initializeCaptureEngine();



window.TabForgeCapture={

startTabCapture,

stopTabCapture

};