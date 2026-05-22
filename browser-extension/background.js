// ====================================================
// TabForge Background Runtime v4.0
// Native tabCapture Controller
// Phase B
// ====================================================

(()=>{

console.log(
"[TABFORGE BG] Native Boot"
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
errors:0,
nativeStreams:0

}

};

const STATE=
globalThis.__TABFORGE_BG__;



// =====================================
// HELPERS
// =====================================

function session(){

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

x=>url.startsWith(x)

);

}



// =====================================
// INSTALL
// =====================================

chrome.runtime.onInstalled.addListener(

()=>{

console.log(
"[INSTALLED]"
);

}

);



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
// AUTO INJECT
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
session();


// Native capture

chrome.tabCapture.capture(

{

audio:true,

video:true,

videoConstraints:{

mandatory:{

maxWidth:1920,

maxHeight:1080,

maxFrameRate:60

}

}

},

async(stream)=>{


if(

chrome.runtime.lastError
||

!stream

){

STATE.metrics.errors++;

console.error(

chrome.runtime.lastError

);

return;

}


STATE.metrics.nativeStreams++;


await chrome.scripting.executeScript({

target:{
tabId
},

func:(tabId)=>{

globalThis
.TabForgeCapture
?.attachNativeStream(
tabId);

},

args:[
String(tabId)
]

});


STATE.active.set(

tabId,

{

sessionId,

stream,

started:
Date.now()

}

);


STATE.metrics.starts++;


console.log(

`[NATIVE STARTED] ${tabId}`

);

}

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

const active=

STATE.active.get(
tabId
);

if(!active){

return;

}


try{


active.stream
?.getTracks()

.forEach(

t=>t.stop()

);


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
// HOTKEY
// =====================================

chrome.commands.onCommand.addListener(

async(command)=>{

const [tab]=

await chrome.tabs.query({

active:true,

currentWindow:true

});


if(
!tab?.id
){

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
// CLEANUP
// =====================================

chrome.tabs.onRemoved.addListener(

tabId=>{

stopCapture(
tabId
);

STATE.injected.delete(
tabId
);

}

);



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

[...STATE.active.keys()],

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