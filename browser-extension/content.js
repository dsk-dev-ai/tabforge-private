// =====================================================
// TabForge Content Bridge v2.0
// Page ↔ Extension ↔ Native Runtime
// Phase A tabcapture-native
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

capture:false,

sessionId:null,

heartbeat:null,

lastURL:location.href,

messageCount:0,

errors:0

};

const STATE=
globalThis.__TABFORGE_CONTENT_STATE__;



// ======================================
// HELPERS
// ======================================

function sid(){

return crypto.randomUUID();

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

captureMode:
"native"

};

}


function send(payload){

try{

STATE.messageCount++;

chrome.runtime.sendMessage(
payload
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
// INITIAL LOAD
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
// SPA NAVIGATION
// ======================================

const observer=

new MutationObserver(()=>{

if(
location.href!==STATE.lastURL
){

STATE.lastURL=
location.href;


send({

action:
"URL_CHANGED",

url:
location.href,

time:
Date.now()

});


console.log(

"[URL UPDATED]",

location.href

);

}

});


observer.observe(

document,

{

subtree:true,

childList:true

}

);




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


console.log(

"[VISIBILITY]",

document.visibilityState

);

}

);



// ======================================
// TAB EXIT
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


// =============================
// START SESSION
// =============================

if(

message.action==="RUN_CAPTURE"

){

if(
STATE.capture
){

sendResponse({

success:true

});

return;

}


STATE.capture=true;

STATE.sessionId=sid();


console.log(

"[CAPTURE ACTIVE]",

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

}



// =============================
// STOP
// =============================

if(

message.action==="STOP_CAPTURE"

){

STATE.capture=false;


send({

action:
"CAPTURE_STOPPED",

sessionId:
STATE.sessionId

});


STATE.sessionId=null;


console.log(
"[CAPTURE STOP]"
);


sendResponse({

success:true

});

}



// =============================
// HEALTH
// =============================

if(

message.action==="PING"

){

sendResponse({

alive:true,

capture:
STATE.capture,

messages:
STATE.messageCount,

errors:
STATE.errors

});

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

capture:
STATE.capture,

time:
Date.now(),

url:
location.href

});

},15000);




// ======================================
// DEBUG
// ======================================

globalThis.TabForgeContent={

stats(){

console.table({

capture:
STATE.capture,

messages:
STATE.messageCount,

errors:
STATE.errors

});

}

};


console.log(
"[TABFORGE CONTENT READY]"
);

})();