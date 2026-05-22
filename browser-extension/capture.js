// =====================================================
// TabForge Capture Runtime v5
// MV3 Stable + Envelope v1
// Background -> StreamId -> Recorder -> WS -> Rust
// =====================================================

(()=>{

try{

if(window.__TABFORGE_CAPTURE__){

return;

}

window.__TABFORGE_CAPTURE__=true;


const WS_URL="ws://127.0.0.1:8765";

const RECORD_INTERVAL=1000;

const VERSION="v1";


const STATE={

sessions:new Map(),
sockets:new Map(),

metrics:{

started:0,
stopped:0,
chunks:0,
bytes:0

}

};


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

version:VERSION,

traceId:id(),

timestamp:Date.now(),

sessionId,

type,

payload

};

}



// =====================================
// REQUEST STREAM ID
// =====================================

async function requestStreamId(tabId){

return new Promise(

(resolve,reject)=>{

chrome.runtime.sendMessage(

{

action:"REQUEST_CAPTURE",

tabId

},

response=>{

if(

chrome.runtime.lastError

){

reject(

chrome.runtime.lastError
.message

);

return;

}


if(

!response ||
!response.streamId

){

reject(

"streamId missing"

);

return;

}


resolve(

response.streamId

);

}

);

}

);

}



// =====================================
// CREATE STREAM
// =====================================

async function createNativeStream(

tabId

){

try{

const streamId=

await requestStreamId(
tabId
);


return navigator

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

}
catch(error){

console.error(

"[STREAM FAIL]",

error

);

return null;

}

}



// =====================================
// SOCKET
// =====================================

async function socket(

sessionId

){

return new Promise(

(resolve,reject)=>{

const ws=

new WebSocket(
WS_URL
);


ws.binaryType=
"arraybuffer";


ws.onopen=()=>{

STATE.sockets.set(
sessionId,
ws
);

resolve(
ws
);

};


ws.onerror=e=>{

reject(e);

};


ws.onclose=()=>{

STATE.sockets.delete(
sessionId
);

};

}

);

}



// =====================================
// START
// =====================================

async function attachNativeStream(

tabId

){

try{

if(

STATE.sessions.has(
tabId
)

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


const sessionId=id();

const ws=

await socket(
sessionId
);


ws.send(

JSON.stringify(

envelope(

sessionId,

"SessionStart",

{

sessionId,
tabId

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

MediaRecorder

.isTypeSupported(

"video/webm;codecs=vp9,opus"

)

?

"video/webm;codecs=vp9,opus"

:

"video/webm"

}

);



recorder.ondataavailable=

async(event)=>{

if(
!event.data?.size
){

return;

}


if(
ws.readyState!==1
){

return;

}


const buffer=

await event
.data
.arrayBuffer();


chunk++;

STATE.metrics
.chunks++;

STATE.metrics
.bytes+=
buffer.byteLength;


ws.send(

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


ws.send(
buffer
);

};


recorder.onstart=()=>{

STATE.metrics
.started++;

console.log(

"[REC START]",

sessionId

);

};


stream

.getTracks()

.forEach(

track=>{

track.onended=()=>{

stopTabCapture(
tabId
);

};

}

);


STATE.sessions.set(

tabId,

{

sessionId,
stream,
ws,
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

"[ATTACH ERROR]",

error

);

}

}



// =====================================
// STOP
// =====================================

function stopTabCapture(

tabId

){

const s=

STATE.sessions.get(
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

x=>x.stop()

);


if(

s.ws?.readyState===1

){

s.ws.send(

JSON.stringify(

envelope(

s.sessionId,

"SessionEnd",

{

sessionId:
s.sessionId

}

)

)

);

s.ws.close();

}


STATE.sessions
.delete(tabId);


STATE.metrics
.stopped++;

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

STATE.metrics

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