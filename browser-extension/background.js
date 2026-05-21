// ====================================================
// TabForge Background Runtime v2.0
// Extension → Runtime → Capture Engine
// Production Multi-Tab Runtime
// ====================================================

(() => {

console.log(
"[TABFORGE] Background runtime booting"
);


// =====================================
// GLOBAL STATE
// =====================================

globalThis.__TABFORGE_BG__ ??= {

injectedTabs:new Set(),

activeCaptures:new Map(),

heartbeat:new Map(),

metrics:{

starts:0,
stops:0,
injects:0,
errors:0

}

};

const STATE=
globalThis.__TABFORGE_BG__;


// =====================================
// INSTALL
// =====================================

chrome.runtime.onInstalled.addListener(()=>{

console.log(
"[TABFORGE] Installed"
);

});


// =====================================
// VALIDATOR
// =====================================

function validTab(tab){

if(
!tab ||
!tab.url
){

return false;

}


const blocked=[

"chrome://",
"edge://",
"devtools://",
"chrome-extension://",
"about:",
"view-source:"

];


return !blocked.some(

prefix=>

tab.url.startsWith(
prefix
)

);

}



// =====================================
// HEARTBEAT
// =====================================

function startHeartbeat(tabId){

stopHeartbeat(
tabId
);

const id=

setInterval(()=>{

console.log(
`[HEARTBEAT] ${tabId}`
);

},15000);


STATE.heartbeat.set(
tabId,
id
);

}


function stopHeartbeat(tabId){

const timer=

STATE.heartbeat.get(
tabId
);

if(timer){

clearInterval(
timer
);

}


STATE.heartbeat.delete(
tabId
);

}



// =====================================
// INJECT RUNTIME
// =====================================

async function injectRuntime(tabId){

if(

STATE
.injectedTabs
.has(tabId)

){

return true;

}


try{

await chrome.scripting.executeScript({

target:{
tabId
},

files:[
"capture.js"
]

});


STATE
.injectedTabs
.add(
tabId
);


STATE
.metrics
.injects++;


console.log(

`[INJECTED] ${tabId}`

);


return true;

}
catch(error){

STATE
.metrics
.errors++;


console.warn(

`[INJECT FAIL] ${tabId}`,

error

);


return false;

}

}



// =====================================
// AUTO INJECTION
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
!validTab(tab)
){

return;

}


await injectRuntime(
tabId
);

}

);



// =====================================
// TAB RELOAD
// =====================================

chrome.tabs.onReplaced.addListener(

(newId,oldId)=>{

STATE
.injectedTabs
.delete(
oldId
);

STATE
.activeCaptures
.delete(
oldId
);

}

);



// =====================================
// TAB CLOSED
// =====================================

chrome.tabs.onRemoved.addListener(

(tabId)=>{

stopHeartbeat(
tabId
);


STATE
.injectedTabs
.delete(
tabId
);

STATE
.activeCaptures
.delete(
tabId
);


console.log(
`[TAB CLOSED] ${tabId}`
);

}

);



// =====================================
// MESSAGE ROUTER
// =====================================

chrome.runtime.onMessage.addListener(

async(
message,
sender,
sendResponse
)=>{

try{


// =================================
// START
// =================================

if(

message.action===
"START_CAPTURE"

){

const tabId=
Number(
message.tabId
);


if(

STATE
.activeCaptures
.has(tabId)

){

console.warn(

`[ACTIVE] ${tabId}`

);


sendResponse({

success:true,

active:true

});

return true;

}


console.log(
`[START] ${tabId}`
);


const ready=

await injectRuntime(
tabId
);


if(!ready){

throw new Error(
"runtime injection failed"
);

}


await chrome.scripting.executeScript({

target:{
tabId
},

func:(tabId)=>{

globalThis
.TabForgeCapture
?.startTabCapture(
String(tabId)
);

},

args:[
tabId
]

});


STATE
.activeCaptures
.set(

tabId,

{

started:
Date.now()

}

);


STATE
.metrics
.starts++;


startHeartbeat(
tabId
);


sendResponse({

success:true

});

}



// =================================
// STOP
// =================================

if(

message.action===
"STOP_CAPTURE"

){

const tabId=
Number(
message.tabId
);


console.log(
`[STOP] ${tabId}`
);


await chrome.scripting.executeScript({

target:{
tabId
},

func:(tabId)=>{

globalThis
.TabForgeCapture
?.stopTabCapture(
String(tabId)
);

},

args:[
tabId
]

});


STATE
.activeCaptures
.delete(
tabId
);


stopHeartbeat(
tabId
);


STATE
.metrics
.stops++;


sendResponse({

success:true

});

}



// =================================
// STATUS
// =================================

if(

message.action===
"RUNTIME_STATS"

){

sendResponse({

metrics:
STATE.metrics,

active:

[
...STATE
.activeCaptures
.keys()

]

});

}


}
catch(error){

STATE
.metrics
.errors++;


console.error(

"[BACKGROUND ERROR]",

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



console.log(
"[TABFORGE] Background ready"
);

})();