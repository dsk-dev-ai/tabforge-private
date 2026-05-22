// =====================================================
// TabForge Content Runtime v3
// MV3 + Phase D Stable Runtime
// =====================================================

(() => {

// ======================================
// DUPLICATE GUARD
// ======================================

if(window.__TABFORGE_CONTENT_LOADED__){

console.log(
"[TABFORGE] already loaded"
);

return;

}

window.__TABFORGE_CONTENT_LOADED__=true;

console.log(
"[TABFORGE] content runtime online"
);


// ======================================
// STATE
// ======================================

window.__TABFORGE_CONTENT_STATE__ ??= {

capture:false,

sessionId:null,

heartbeat:null,

observer:null,

lastURL:location.href,

messageCount:0,

errors:0,

alive:true

};

const STATE=
window.__TABFORGE_CONTENT_STATE__;


// ======================================
// HELPERS
// ======================================

function sid(){

return crypto.randomUUID();

}


function extensionAlive(){

try{

return !!chrome?.runtime?.id;

}
catch{

return false;

}

}


function metadata(){

return{

sessionId:
STATE.sessionId,

title:
document.title,

url:
location.href,

visibility:
document.visibilityState,

ready:
document.readyState,

focused:
document.hasFocus(),

timestamp:
Date.now(),

capture:
STATE.capture,

captureMode:
"native"

};

}



// ======================================
// SAFE SEND
// ======================================

function send(payload){

try{

if(
!extensionAlive()
){

STATE.alive=false;

return;

}


STATE.messageCount++;


chrome.runtime.sendMessage(

payload,

(response)=>{

if(
chrome.runtime.lastError
){

STATE.errors++;

console.warn(

"[SEND ERROR]",

chrome.runtime.lastError.message

);

return;

}

}

);

}
catch(error){

STATE.errors++;

console.warn(

"[CONTENT SEND ERROR]",

error

);

}

}



// ======================================
// PAGE READY
// ======================================

window.addEventListener(

"load",

()=>{

send({

action:
"TAB_METADATA",

payload:
metadata()

});


console.log(
"[TAB READY]"
);

}

);




// ======================================
// SPA WATCH
// ======================================

STATE.observer=

new MutationObserver(()=>{

if(
location.href===
STATE.lastURL
){

return;

}


STATE.lastURL=
location.href;


send({

action:
"URL_CHANGED",

url:
location.href,

title:
document.title,

time:
Date.now()

});


console.log(

"[URL UPDATED]",

location.href

);

});


if(document.body){

STATE.observer.observe(

document.body,

{

subtree:true,

childList:true

}

);

}



// ======================================
// VISIBILITY
// ======================================

document.addEventListener(

"visibilitychange",

()=>{

send({

action:
"TAB_VISIBILITY",

visibility:
document.visibilityState,

url:
location.href

});

});




// ======================================
// EXIT
// ======================================

window.addEventListener(

"beforeunload",

()=>{

send({

action:
"TAB_UNLOAD",

url:
location.href,

sessionId:
STATE.sessionId

});


if(
STATE.heartbeat
){

clearInterval(
STATE.heartbeat
);

}

});




// ======================================
// MESSAGE ROUTER
// ======================================

chrome.runtime.onMessage.addListener(

async(
message,
sender,
sendResponse
)=>{

try{


// =====================
// START
// =====================

if(

message.action===
"RUN_CAPTURE"

){

if(
STATE.capture
){

sendResponse({

success:true,

active:true

});

return true;

}


STATE.capture=true;

STATE.sessionId=
sid();


console.log(

"[CAPTURE START]",

STATE.sessionId

);


send({

action:
"CAPTURE_STARTED",

sessionId:
STATE.sessionId

});


sendResponse({

success:true

});

return true;

}



// =====================
// STOP
// =====================

if(

message.action===
"STOP_CAPTURE"

){

STATE.capture=false;


send({

action:
"CAPTURE_STOPPED",

sessionId:
STATE.sessionId

});


console.log(

"[CAPTURE STOP]",

STATE.sessionId

);


STATE.sessionId=null;


sendResponse({

success:true

});

return true;

}



// =====================
// HEALTH
// =====================

if(

message.action===
"PING"

){

sendResponse({

alive:
extensionAlive(),

capture:
STATE.capture,

session:
STATE.sessionId,

messages:
STATE.messageCount,

errors:
STATE.errors,

url:
location.href

});

return true;

}

}
catch(error){

STATE.errors++;

console.error(

"[CONTENT ERROR]",

error

);


sendResponse({

success:false,

error:
String(error)

});

}

return true;

}

);




// ======================================
// HEARTBEAT
// ======================================

STATE.heartbeat=

setInterval(()=>{

if(
!extensionAlive()
){

clearInterval(
STATE.heartbeat
);

return;

}


send({

action:
"HEARTBEAT",

capture:
STATE.capture,

sessionId:
STATE.sessionId,

time:
Date.now(),

url:
location.href

});

},15000);




// ======================================
// DEBUG
// ======================================

window.TabForgeContent={

stats(){

console.table({

capture:
STATE.capture,

session:
STATE.sessionId,

messages:
STATE.messageCount,

errors:
STATE.errors,

url:
location.href

});

}

};


console.log(
"[TABFORGE CONTENT READY]"
);

})();