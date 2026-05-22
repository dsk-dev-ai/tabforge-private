// ====================================================
// TabForge Tab Stream Bridge v4
// Popup → Background → Capture Runtime
// Phase D Protocol Envelope
// ====================================================

(()=>{

if(window.__TABFORGE_STREAM_LOADED__){

console.log(
"[TABSTREAM] already loaded"
);

return;

}

window.__TABFORGE_STREAM_LOADED__=true;


// =====================================
// STATE
// =====================================

window.__TABFORGE_STREAM_STATE__ ??= {

sent:0,
errors:0,
duplicates:0,
lastPayload:null,
lastHash:null

};

const STATE=
window.__TABFORGE_STREAM_STATE__;

const PROTOCOL_VERSION=
"v1";



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
"about:",
"devtools://",
"chrome-extension://"

]

.some(

v=>url.startsWith(v)

);

}


function createEnvelope(

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

sessionId:
id(),

type,

payload

};

}



// =====================================
// TAB FILTER
// =====================================

function sanitizeTabs(tabs){

if(

!Array.isArray(
tabs
)

){

return [];

}


return tabs

.filter(

tab=>

tab &&
tab.id &&
tab.url &&
tab.title &&
!blocked(
tab.url
)

)

.map(

tab=>({

id:
String(tab.id),

title:
tab.title,

url:
tab.url,

windowId:
tab.windowId,

active:
tab.active ?? false,

captureMode:
"native"

})

);

}



// =====================================
// DEDUPE
// =====================================

function hash(v){

return JSON.stringify(v);

}



// =====================================
// SEND
// =====================================

function sendTabsToTabForge(tabs){

try{

const cleaned=

sanitizeTabs(
tabs
);


if(
!cleaned.length
){

return;

}


const payload=

createEnvelope(

"SessionMeta",

{

selected_tabs:
cleaned,

count:
cleaned.length

}

);


const currentHash=

hash(
payload.payload
);


if(

STATE.lastHash===currentHash

){

STATE.duplicates++;

console.log(
"[TABSTREAM] duplicate"
);

return;

}


STATE.lastHash=
currentHash;

STATE.lastPayload=
payload;

STATE.sent++;


chrome.runtime.sendMessage(

{

action:
"TAB_PAYLOAD",

payload

},

response=>{

if(

chrome.runtime.lastError

){

STATE.errors++;

console.error(

"[BRIDGE ERROR]",

chrome.runtime.lastError
.message

);

return;

}


console.log(

"[BRIDGE ACK]",

response

);

}

);


console.log(

"[TAB PAYLOAD]",

payload

);

}
catch(error){

STATE.errors++;

console.error(

"[TABSTREAM ERROR]",

error

);

}

}



// =====================================
// DEBUG
// =====================================

window.TabForgeTabStream={

sendTabsToTabForge,

stats(){

console.table({

sent:
STATE.sent,

errors:
STATE.errors,

duplicates:
STATE.duplicates

});

},


health(){

return{

online:true,

sent:
STATE.sent,

errors:
STATE.errors

};

},


last(){

console.log(

STATE.lastPayload

);

}

};


window.sendTabsToTabForge=

sendTabsToTabForge;


console.log(
"[TABSTREAM READY]"
);

})();