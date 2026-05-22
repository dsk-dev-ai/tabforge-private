// =====================================================
// TabForge Capture Runtime v4
// MV3 + Protocol Envelope v1
// TabCapture → MediaRecorder → WS → Rust
// =====================================================

(() => {

try{

if(window.__TABFORGE_CAPTURE_RUNTIME__){

console.log(
"[TABFORGE] capture already loaded"
);

return;

}

window.__TABFORGE_CAPTURE_RUNTIME__=true;


// =====================================
// STATE
// =====================================

window.__TABFORGE_CAPTURE_STATE__ ??= {

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
window.__TABFORGE_CAPTURE_STATE__;

const SESSIONS=
STATE.sessions;

const SOCKETS=
STATE.sockets;

const METRICS=
STATE.metrics;


const WS_URL=
"ws://127.0.0.1:8765";

const PROTOCOL_VERSION=
"v1";

const RECORD_INTERVAL=
1000;



// =====================================
// HELPERS
// =====================================

function id(){

return crypto.randomUUID();

}


function envelope(

sessionId,
type,
payload

){

return{

version:
PROTOCOL_VERSION,

traceId:
id(),

timestamp:
Date.now(),

sessionId,

type,

payload

};

}



// =====================================
// SOCKET
// =====================================

async function createSocket(sessionId){

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
"[WS CONNECTED]"
);

resolve(ws);

};


ws.onerror=(e)=>{

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



// =====================================
// TAB STREAM
// =====================================

async function createNativeStream(tabId){

try{

const streamId=

await chrome.tabCapture
.getMediaStreamId({

targetTabId:
tabId

});


const stream=

await navigator
.mediaDevices
.getUserMedia({

audio:{

mandatory:{

chromeMediaSource:
"tab",

chromeMediaSourceId:
streamId

}

},

video:{

mandatory:{

chromeMediaSource:
"tab",

chromeMediaSourceId:
streamId,

maxWidth:1920,

maxHeight:1080,

maxFrameRate:60

}

}

});


return stream;

}
catch(error){

console.error(

"[STREAM ERROR]",

error

);

return null;

}

}



// =====================================
// START
// =====================================

async function attachNativeStream(tabId){

try{

if(
SESSIONS.has(tabId)
){

return;

}


const stream=

await createNativeStream(
tabId
);


if(!stream){

return;

}


const sessionId=
id();

const socket=

await createSocket(
sessionId
);


socket.send(

JSON.stringify(

envelope(

sessionId,

"SessionStart",

{

session_id:
sessionId,

tab_id:
tabId,

chunk:0,

time:
Date.now(),

size:0

}

)

)

);


let chunk=0;


const recorder=

new MediaRecorder(

stream,

{

mimeType:
"video/webm;codecs=vp9"

}

);


recorder.onstart=()=>{

METRICS.started++;

console.log(

"[REC START]",

sessionId

);

};


recorder.ondataavailable=

async(event)=>{

if(
!event.data.size
){

return;

}


if(
socket.readyState!==1
){

return;

}


const buffer=

await event
.data
.arrayBuffer();


chunk++;

METRICS.chunks++;

METRICS.bytes+=
buffer.byteLength;


socket.send(

JSON.stringify(

envelope(

sessionId,

"ChunkPacket",

{

chunk,

size:
buffer.byteLength

}

)

)

);


socket.send(
buffer
);


if(
chunk%10===0
){

console.log(

"[CHUNK]",

chunk

);

}

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

socket,

stream,

recorder

}

);


recorder.start(
RECORD_INTERVAL
);


console.log(

"[LIVE]",

tabId

);

}
catch(error){

console.error(

"[CAPTURE ERROR]",

error

);

}

}



// =====================================
// STOP
// =====================================

function stopTabCapture(tabId){

const session=

SESSIONS.get(
tabId
);


if(
!session
){

return;

}


try{

session.recorder
?.stop();


session.stream
?.getTracks()

.forEach(

track=>track.stop()

);


if(

session.socket
?.readyState===1

){

session.socket.send(

JSON.stringify(

envelope(

session.sessionId,

"SessionEnd",

{

session_id:
session.sessionId

}

)

)

);


session.socket.close();

}


SOCKETS.delete(

session.sessionId

);


SESSIONS.delete(
tabId
);


METRICS.stopped++;


console.log(

"[STOP]",

tabId

);

}
catch(error){

console.error(

"[STOP ERROR]",

error

);

}

}



// =====================================
// DEBUG
// =====================================

window.TabForgeCapture={

attachNativeStream,

stopTabCapture,

stats(){

console.table(
METRICS
);

}

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