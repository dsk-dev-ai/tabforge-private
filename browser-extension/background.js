// ====================================================
// TabForge Background Runtime v3.0
// Native Capture Controller
// Phase A tabcapture-native
// ====================================================

(()=>{

console.log(
"[TABFORGE BG] Boot"
);


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

errors:0

}

};

const STATE=
globalThis.__TABFORGE_BG__;



// =====================================
// HELPERS
// =====================================

function id(){

return crypto.randomUUID();

}


function blocked(url){

if(!url){

return true;

}

return [

"chrome://",

"edge://",

"devtools://",

"chrome-extension://",

"view-source:",

"about:"

]

.some(

v=>url.startsWith(v)

);

}



// =====================================
// INSTALL
// =====================================

chrome.runtime.onInstalled.addListener(()=>{

console.log(
"[INSTALLED]"
);

});



// =====================================
// INJECT
// =====================================

async function inject(tabId){

if(
STATE.injected.has(tabId)
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


STATE.injected.add(
tabId
);

STATE.metrics.injects++;


console.log(
`[INJECTED] ${tabId}`
);

return true;

}
catch(error){

STATE.metrics.errors++;

console.error(
error
);

return false;

}

}



// =====================================
// AUTO
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
blocked(tab?.url)
){

return;

}

await inject(
tabId
);

}

);



// =====================================
// CLEANUP
// =====================================

chrome.tabs.onRemoved.addListener(

tabId=>{

STATE.injected.delete(
tabId
);

STATE.active.delete(
tabId
);

console.log(
`[REMOVED] ${tabId}`
);

}

);



// =====================================
// HOTKEYS
// =====================================

chrome.commands.onCommand.addListener(

async(command)=>{

const [tab]=

await chrome.tabs.query({

active:true,

currentWindow:true

});


if(!tab?.id){

return;

}


if(

command==="start_capture"

){

startCapture(
tab.id
);

}


if(

command==="stop_capture"

){

stopCapture(
tab.id
);

}

});



// =====================================
// START
// =====================================

async function startCapture(tabId){

try{

if(
STATE.active.has(tabId)
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


const sessionId=
id();


await chrome.scripting.executeScript({

target:{
tabId
},

func:(tabId)=>{

globalThis
.TabForgeCapture
?.startTabCapture(
tabId
);

},

args:[
String(tabId)
]

});


STATE.active.set(

tabId,

{

sessionId,

started:
Date.now()

}

);


STATE.metrics.starts++;


console.log(

`[STARTED] ${tabId}`

);

}
catch(error){

STATE.metrics.errors++;

console.error(
error
);

}

}



// =====================================
// STOP
// =====================================

async function stopCapture(tabId){

try{

await chrome.scripting.executeScript({

target:{
tabId
},

func:(tabId)=>{

globalThis
.TabForgeCapture
?.stopTabCapture(
tabId
);

},

args:[
String(tabId)
]

});


STATE.active.delete(
tabId
);

STATE.metrics.stops++;


console.log(

`[STOPPED] ${tabId}`

);

}
catch(error){

STATE.metrics.errors++;

}

}



// =====================================
// ROUTER
// =====================================

chrome.runtime.onMessage.addListener(

async(
message,
sender,
sendResponse
)=>{

try{


if(

message.action==="START_CAPTURE"

){

await startCapture(

Number(
message.tabId
)

);


sendResponse({

success:true

});

}



if(

message.action==="STOP_CAPTURE"

){

await stopCapture(

Number(
message.tabId
)

);


sendResponse({

success:true

});

}



if(

message.action==="RUNTIME_STATS"

){

sendResponse({

active:

[

...STATE
.active
.keys()

],

metrics:
STATE.metrics

});

}

}
catch(error){

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
"[TABFORGE BG READY]"
);

})();