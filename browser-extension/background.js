// ====================================================
// TabForge Background Runtime v5
// MV3 Stable Runtime + Capture Ownership
// ====================================================

(()=>{

console.log("[TABFORGE BG] boot");

if(globalThis.__TABFORGE_BG_RUNTIME__){

return;

}

globalThis.__TABFORGE_BG_RUNTIME__=true;


// =====================================
// STATE
// =====================================

globalThis.__TABFORGE_BG__ ??= {

injected:new Set(),

active:new Map(),

metrics:{

starts:0,
stops:0,
injects:0,
errors:0,
captureRequests:0

}

};

const STATE=
globalThis.__TABFORGE_BG__;



// =====================================
// HELPERS
// =====================================

function blocked(url){

if(!url){

return true;

}

return[

"chrome://",
"edge://",
"chrome-extension://",
"devtools://",
"about:",
"view-source:"

]

.some(

v=>url.startsWith(v)

);

}



// =====================================
// INJECT
// =====================================

async function inject(tabId){

try{

if(

STATE.injected.has(
tabId
)

){

return true;

}


await chrome.scripting.executeScript({

target:{
tabId
},

files:[

"capture.js",
"content.js"

]

});


STATE.injected.add(
tabId
);


STATE.metrics.injects++;


console.log(

"[INJECTED]",

tabId

);

return true;

}
catch(error){

STATE.metrics.errors++;

console.error(

"[INJECT ERROR]",

error

);

return false;

}

}



// =====================================
// TAB UPDATES
// =====================================

chrome.tabs.onUpdated.addListener(

async(

tabId,
info,
tab

)=>{

if(

info.status!=="complete"

){

return;

}


if(

blocked(
tab?.url
)

){

return;

}


await inject(
tabId
);

}

);



// =====================================
// START
// =====================================

async function startCapture(

tabId

){

try{

if(

STATE.active.has(
tabId
)

){

return;

}


const ok=

await inject(
tabId
);

if(!ok){

return;

}


await chrome.tabs.sendMessage(

tabId,

{

action:
"RUN_CAPTURE"

}

);


STATE.active.set(

tabId,

{

started:
Date.now()

}

);


STATE.metrics.starts++;


console.log(

"[STARTED]",

tabId

);

}
catch(error){

STATE.metrics.errors++;

console.error(

"[START ERROR]",

error

);

}

}



// =====================================
// STOP
// =====================================

async function stopCapture(

tabId

){

try{

await chrome.tabs.sendMessage(

tabId,

{

action:
"STOP_CAPTURE"

}

);


STATE.active.delete(
tabId
);

STATE.metrics.stops++;


console.log(

"[STOPPED]",

tabId

);

}
catch(error){

STATE.metrics.errors++;

console.error(

"[STOP ERROR]",

error

);

}

}



// =====================================
// ROUTER
// =====================================

chrome.runtime.onMessage.addListener(

(

message,
sender,
sendResponse

)=>{


// =====================================
// MV3 STREAM REQUEST
// =====================================

if(

message.action==="REQUEST_CAPTURE"

){

STATE.metrics
.captureRequests++;


chrome.tabCapture
.getMediaStreamId(

{

targetTabId:

message.tabId

},

streamId=>{


if(

chrome.runtime
.lastError

){

sendResponse({

success:false,

error:

chrome.runtime
.lastError
.message

});

return;

}


sendResponse({

success:true,

streamId

});

}

);

return true;

}



// =====================================
// START
// =====================================

if(

message.action==="START_CAPTURE"

){

startCapture(

Number(
message.tabId
)

);

sendResponse({

success:true

});

return true;

}



// =====================================
// STOP
// =====================================

if(

message.action==="STOP_CAPTURE"

){

stopCapture(

Number(
message.tabId
)

);

sendResponse({

success:true

});

return true;

}



// =====================================
// STATS
// =====================================

if(

message.action==="RUNTIME_STATS"

){

sendResponse({

active:

[

...STATE.active
.keys()

],

metrics:

STATE.metrics

});

return true;

}

}

);




// =====================================
// CLEANUP
// =====================================

chrome.tabs.onRemoved.addListener(

tabId=>{

STATE.active.delete(
tabId
);

STATE.injected.delete(
tabId
);

}

);



console.log(
"[TABFORGE BG READY]"
);

})();