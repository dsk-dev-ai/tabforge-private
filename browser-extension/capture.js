// TabForge Real Browser Stream Engine

const activeCaptures = new Map();

const sockets = new Map();

const WS_URL =
    "ws://127.0.0.1:8765";


async function initializeCaptureEngine(){

    console.log(
        "[TABFORGE] Capture engine online"
    );

    console.log(
        "[TABFORGE] WebSocket target:",
        WS_URL
    );

}


async function createSocket(
    tabId
){

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


            socket.onerror=(e)=>{

                console.error(
                    `[SOCKET] Failed ${tabId}`,
                    e
                );

                reject(e);

            };


            socket.onclose=()=>{

                console.log(
                    `[SOCKET] Closed ${tabId}`
                );

                sockets.delete(
                    tabId
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
"No stream returned"
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
"video/webm;codecs=vp9,opus"

}

);



recorder.ondataavailable=

async(event)=>{


if(

event.data.size===0

||

socket.readyState!==1

){

return;

}


const buffer=

await event
.data
.arrayBuffer();


const metadata={

tabId,

timestamp:
Date.now(),

type:
"video/webm"

};


socket.send(

JSON.stringify(
metadata
)

);


socket.send(
buffer
);


console.log(

`[CHUNK] ${tabId} → ${event.data.size}`

);

};



recorder.onstart=()=>{

console.log(
`[RECORDER] ${tabId} active`
);

};


recorder.onstop=()=>{

console.log(
`[RECORDER] ${tabId} stopped`
);

};


recorder.start(
1000
);


activeCaptures.set(

tabId,

{

stream,

recorder,

socket

}

);

}
catch(error){

console.error(

`[CAPTURE ERROR] ${tabId}`,

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
capture.socket
){

capture.socket.close();

}


activeCaptures.delete(
tabId
);


console.log(
`[CAPTURE] Released ${tabId}`
);

}



initializeCaptureEngine();


window.TabForgeCapture={

startTabCapture,

stopTabCapture

};