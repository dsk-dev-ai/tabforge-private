// =====================================================
// TabForge Runtime Capture Engine v1.1
// Browser → MediaRecorder → WebSocket → Rust
// Stable Runtime
// =====================================================

(()=>{

try{

if(globalThis.__TABFORGE_RUNTIME__){

console.log(
"[TABFORGE] Runtime already active"
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

heartbeats:new Map(),

metrics:{

started:0,
stopped:0,
chunks:0,
bytes:0,
reconnects:0

}

};

const STATE=
globalThis.__TABFORGE_STATE__;

const SESSIONS=
STATE.sessions;

const SOCKETS=
STATE.sockets;

const HEARTBEATS=
STATE.heartbeats;

const METRICS=
STATE.metrics;


const WS_URL=
"ws://127.0.0.1:8765";

const RECORD_INTERVAL=1000;

const WS_TIMEOUT=5000;

const HEARTBEAT=10000;

const MAX_RETRIES=3;



function now(){

return Date.now();

}


function uuid(){

return crypto.randomUUID();

}



// ======================================
// SOCKET
// ======================================

async function createSocket(

sessionId,

retry=0

){

const existing=

SOCKETS.get(
sessionId
);


if(

existing &&
existing.readyState===1

){

return existing;

}


return new Promise(

(resolve,reject)=>{

const socket=

new WebSocket(
WS_URL
);

socket.binaryType=
"arraybuffer";


const timeout=

setTimeout(()=>{

socket.close();

reject(

new Error(
"WS timeout"
)

);

},WS_TIMEOUT);


socket.onopen=()=>{

clearTimeout(
timeout
);

SOCKETS.set(
sessionId,
socket
);


console.log(
`[WS OPEN] ${sessionId}`
);


const interval=

setInterval(()=>{

try{

if(

socket.readyState===1

){

socket.send(

JSON.stringify({

type:"heartbeat",

time:now()

})

);

}

}
catch{}

},HEARTBEAT);


HEARTBEATS.set(
sessionId,
interval
);


resolve(
socket
);

};



socket.onmessage=(m)=>{

console.log(
"[RUST]",
m.data
);

};


socket.onerror=(e)=>{

console.warn(
"[WS ERROR]",
e
);

};


socket.onclose=async()=>{

console.log(
`[WS CLOSED] ${sessionId}`
);


clearInterval(

HEARTBEATS.get(
sessionId
)

);

HEARTBEATS.delete(
sessionId);

SOCKETS.delete(
sessionId
);


if(

SESSIONS.has(sessionId)
&&
retry<MAX_RETRIES

){

METRICS.reconnects++;

console.log(
"[WS RETRY]"
);

try{

await createSocket(
sessionId,
retry+1
);

}
catch{}

}

};

});

}



// ======================================
// START
// ======================================

async function startTabCapture(tabId){

try{

const existing=

SESSIONS.get(
tabId
);


if(existing){

console.warn(
"[SKIP ACTIVE]"
);

return;

}


const sessionId=
uuid();


SESSIONS.set(

tabId,

{

starting:true,

sessionId

}

);


console.log(
`[START] ${tabId}`
);


const socket=

await createSocket(
sessionId
);


const stream=

await navigator
.mediaDevices
.getDisplayMedia({

video:{

frameRate:60

},

audio:true

});


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

stream,

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

try{

if(

!e.data ||
e.data.size===0

){

return;

}


if(

socket.readyState!==1

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


socket.send(

JSON.stringify({

type:"meta",

sessionId,

tabId,

chunk,

size:
buffer.byteLength,

time:
now()

})

);


socket.send(
buffer
);


if(chunk%10===0){

console.log(

`[STREAM] chunks=${chunk}`

);

}

}
catch(error){

console.error(
"[CHUNK]",
error
);

}

};


recorder.onstop=()=>{

console.log(
"[RECORDER STOP]"
);

};


stream
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

sessionId,

stream,

socket,

recorder,

started:
now()

}

);


recorder.start(
RECORD_INTERVAL
);


console.log(
`[LIVE] ${tabId}`
);

}
catch(error){

SESSIONS.delete(
tabId
);

console.error(
"[START FAILED]",
error
);

}

}



// ======================================
// STOP
// ======================================

function stopTabCapture(tabId){

const session=

SESSIONS.get(
tabId
);


if(!session){

return;

}


try{

if(

session.recorder &&
session.recorder.state!=="inactive"

){

session.recorder.stop();

}


session.stream
?.getTracks()

.forEach(

t=>t.stop()

);


if(

session.socket &&
session.socket.readyState===1

){

session.socket.send(

JSON.stringify({

type:"session_end",

sessionId:
session.sessionId

})

);

session.socket.close();

}


clearInterval(

HEARTBEATS.get(
session.sessionId
)

);


HEARTBEATS.delete(
session.sessionId
);

SOCKETS.delete(
session.sessionId
);

SESSIONS.delete(
tabId
);


METRICS.stopped++;


console.log(
`[STOPPED] ${tabId}`
);

}
catch(error){

console.error(
"[STOP ERROR]",
error
);

}

}



// ======================================
// DEBUG
// ======================================

function sessions(){

console.table(

[...SESSIONS.keys()]

);

}


function stats(){

console.table(
METRICS
);

}


function inspect(){

console.log(
STATE
);

}


globalThis.TabForgeCapture={

startTabCapture,
stopTabCapture,

sessions,
stats,
inspect

};


console.log(
"[TABFORGE API READY]"
);

}
catch(error){

console.error(
"[TABFORGE FATAL]",
error
);

}

})();