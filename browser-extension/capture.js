// =====================================================
// TabForge Native Capture Runtime v3.0
// Stream Attach → MediaRecorder → WS → Rust
// Phase B
// =====================================================

(()=>{

try{

if(globalThis.__TABFORGE_RUNTIME__){

console.log(
"[TABFORGE] Runtime loaded"
);

return;

}

globalThis.__TABFORGE_RUNTIME__=true;


// ======================================
// STATE
// ======================================

globalThis.__TABFORGE_STATE__ ??= {

sessions:new Map(),

sockets:new Map(),

metrics:{

started:0,
stopped:0,
chunks:0,
bytes:0

}

};

const STATE=
globalThis.__TABFORGE_STATE__;

const SESSIONS=
STATE.sessions;

const SOCKETS=
STATE.sockets;

const METRICS=
STATE.metrics;


const WS_URL=
"ws://127.0.0.1:8765";

const RECORD_INTERVAL=1000;



function now(){

return Date.now();

}


function sessionId(){

return crypto.randomUUID();

}



// ======================================
// SOCKET
// ======================================

async function createSocket(id){

const existing=

SOCKETS.get(id);

if(

existing &&
existing.readyState===1

){

return existing;

}


return new Promise(

(resolve,reject)=>{

const ws=

new WebSocket(
WS_URL
);

ws.binaryType=
"arraybuffer";


ws.onopen=()=>{

SOCKETS.set(
id,
ws
);

resolve(
ws
);

};


ws.onerror=e=>{

console.error(
"[WS ERROR]",
e
);

reject(e);

};


ws.onclose=()=>{

SOCKETS.delete(
id
);

};

});

}



// ======================================
// ATTACH
// ======================================

async function attachNativeStream(tabId){

try{

if(
SESSIONS.has(tabId)
){

return;
}


const media=

await navigator.mediaDevices
.getUserMedia({

audio:true,

video:true

});


const id=
sessionId();

const ws=

await createSocket(
id
);


const mime=

MediaRecorder
.isTypeSupported(

"video/webm;codecs=vp9,opus"

)

?

"video/webm;codecs=vp9,opus"

:

"video/webm";


let chunk=0;


const recorder=

new MediaRecorder(

media,

{

mimeType:mime,

videoBitsPerSecond:
8000000,

audioBitsPerSecond:
320000

}

);


recorder.onstart=()=>{

METRICS.started++;

console.log(
"[RECORDER START]"
);

};


recorder.ondataavailable=

async(e)=>{

if(
!e.data ||
e.data.size===0
){

return;

}


if(
ws.readyState!==1
){

return;

}


const buffer=

await e
.data
.arrayBuffer();


chunk++;

METRICS.chunks++;

METRICS.bytes+=
buffer.byteLength;


ws.send(

JSON.stringify({

type:"meta",

capture:"native",

sessionId:id,

tabId,

chunk,

time:now(),

size:
buffer.byteLength

})

);


ws.send(
buffer
);


if(chunk%10===0){

console.log(

`[STREAM ${chunk}]`

);

}

};


media
.getTracks()

.forEach(

track=>{

track.addEventListener(

"ended",

()=>{

stopTabCapture(
tabId
);

}

);

}

);


SESSIONS.set(

tabId,

{

stream:media,

socket:ws,

recorder,

sessionId:id

}

);


recorder.start(
RECORD_INTERVAL
);


console.log(
`[LIVE ${tabId}]`
);

}
catch(error){

console.error(
"[ATTACH ERROR]",
error
);

}

}



// ======================================
// STOP
// ======================================

function stopTabCapture(tabId){

const s=

SESSIONS.get(
tabId
);

if(!s){

return;

}


try{

s.recorder?.stop();


s.stream
?.getTracks()

.forEach(

t=>t.stop()

);


if(
s.socket?.readyState===1
){

s.socket.send(

JSON.stringify({

type:"session_end",

sessionId:
s.sessionId

})

);

s.socket.close();

}


SESSIONS.delete(
tabId
);

METRICS.stopped++;

}
catch(error){

console.error(
"[STOP]",
error
);

}

}



// ======================================
// DEBUG
// ======================================

function stats(){

console.table(
METRICS
);

}


globalThis.TabForgeCapture={

attachNativeStream,

stopTabCapture,

stats

};


console.log(
"[CAPTURE READY]"
);

}
catch(error){

console.error(
"[FATAL]",
error
);

}

})();