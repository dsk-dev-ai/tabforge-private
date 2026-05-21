// =====================================================
// TabForge Content Bridge v1.0
// Page Runtime ↔ Extension ↔ Capture Runtime
// Production Runtime
// =====================================================

(() => {

if(globalThis.__TABFORGE_CONTENT_LOADED__){

console.log(
"[TABFORGE] Content already loaded"
);

return;

}

globalThis.__TABFORGE_CONTENT_LOADED__=true;


console.log(
"[TABFORGE] Content bridge online"
);


// ======================================
// STATE
// ======================================

globalThis.__TABFORGE_CONTENT_STATE__ ??= {

captureRunning:false,

lastPing:0,

heartbeat:null

};

const STATE=
globalThis.__TABFORGE_CONTENT_STATE__;



// ======================================
// METADATA
// ======================================

function getTabMetadata(){

return{

title:
document.title,

url:
location.href,

timestamp:
Date.now(),

visibility:
document.visibilityState,

ready:
document.readyState,

focused:
document.hasFocus()

};

}



// ======================================
// SEND SAFE
// ======================================

function send(message){

try{

chrome.runtime.sendMessage(
message
);

}
catch(error){

console.warn(
"[MESSAGE FAILED]",
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

const metadata=

getTabMetadata();


console.log(

"[TAB INFO]",

metadata

);


send({

action:
"TAB_METADATA",

payload:
metadata

});

}

);



// ======================================
// URL CHANGE
// SPA support
// ======================================

let lastURL=
location.href;

setInterval(()=>{

if(
location.href!==lastURL
){

lastURL=
location.href;


send({

action:
"URL_CHANGED",

url:
location.href

});


console.log(
"[URL CHANGED]",
location.href
);

}

},2000);



// ======================================
// VISIBILITY
// ======================================

document.addEventListener(

"visibilitychange",

()=>{

console.log(

"[VISIBILITY]",

document.visibilityState

);


send({

action:
"TAB_VISIBILITY",

visibility:
document.visibilityState,

url:
location.href

});

}

);



// ======================================
// TAB CLOSE
// ======================================

window.addEventListener(

"beforeunload",

()=>{

send({

action:
"TAB_UNLOAD",

url:
location.href

});

}

);



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


// ------------------------
// START
// ------------------------

if(

message.action===
"RUN_CAPTURE"

){

if(
STATE.captureRunning
){

console.warn(
"[CAPTURE ACTIVE]"
);

sendResponse({

success:true

});

return true;

}


if(

!globalThis.TabForgeCapture

){

throw new Error(
"Capture runtime unavailable"
);

}


console.log(
"[CAPTURE START]"
);


STATE.captureRunning=true;


await globalThis
.TabForgeCapture
.startTabCapture(

String(
message.tabId
)

);


sendResponse({

success:true

});

}



// ------------------------
// STOP
// ------------------------

if(

message.action===
"STOP_CAPTURE"

){

if(

globalThis
.TabForgeCapture

){

globalThis
.TabForgeCapture
.stopTabCapture(

String(
message.tabId
)

);

}


STATE.captureRunning=false;


console.log(
"[CAPTURE STOP]"
);


sendResponse({

success:true

});

}



// ------------------------
// PING
// ------------------------

if(

message.action===
"PING"

){

sendResponse({

alive:true,

capture:
STATE.captureRunning

});

}

}
catch(error){

console.error(

"[CONTENT ERROR]",

error

);


sendResponse({

success:false,

error:
error.toString()

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

send({

action:
"HEARTBEAT",

timestamp:
Date.now(),

url:
location.href

});

},30000);




console.log(
"[TABFORGE] Content ready"
);

})();