// =====================================================
// TabForge Native Capture Runtime v2.0
// chrome.tabCapture → MediaRecorder → WS → Rust
// Phase A
// =====================================================

(()=>{

try{

if(globalThis.__TABFORGE_RUNTIME__){

console.log(
"[TABFORGE] Runtime already loaded"
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

function id(){

return crypto.randomUUID();

}



// ======================================
// SOCKET
// ======================================

async function socket(sessionId){

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

const ws=

new WebSocket(
WS_URL
);

ws.binaryType=
"arraybuffer";


ws.onopen=()=>{

SOCKETS.set(
sessionId,
ws
);

console.log(
"[WS OPEN]"
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
sessionId
);

console.log(
"[WS CLOSED]"
);

};

}

);

}



// ======================================
// START
// ======================================

async function startTabCapture(tabId){

try{

if(

SESSIONS.has(tabId)

){

console.warn(
"[ACTIVE]"
);

return;

}


const sessionId=
id();

console.log(
"[START]"
);


const ws=

await socket(
sessionId
);


// native extension capture

chrome.tabCapture.capture(

{

audio:true,

video:true

},

stream=>{

if(

chrome.runtime.lastError

){

console.error(

chrome.runtime.lastError

);

return;

}


if(!stream){

console.error(
"[NO STREAM]"
);

return;

}


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
8000000

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

capture:
"native",

sessionId,

tabId,

chunk,

time:
now(),

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


stream
.getTracks()

.forEach(track=>{

track.addEventListener(

"ended",

()=>{

stopTabCapture(
tabId
);

}

);

});


SESSIONS.set(

tabId,

{

sessionId,

stream,

recorder,

socket:ws

}

);


recorder.start(
RECORD_INTERVAL
);


console.log(
"[LIVE]"

);

});

}
catch(error){

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


console.log(
"[STOPPED]"
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

function stats(){

console.table(
METRICS
);

}


globalThis.TabForgeCapture={

startTabCapture,
stopTabCapture,
stats

};


console.log(
"[TABFORGE API READY]"
);

}
catch(error){

console.error(
"[FATAL]",
error
);

}

})();