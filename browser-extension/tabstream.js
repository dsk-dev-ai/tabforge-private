// ====================================================
// TabForge Tab Stream Bridge v3.0
// Popup → Background → Native Capture Runtime
// Phase A: tabcapture-native
// ====================================================

(() => {

if(globalThis.__TABFORGE_STREAM_LOADED__){

console.log(
"[TABSTREAM] Already loaded"
);

return;

}

globalThis.__TABFORGE_STREAM_LOADED__=true;


// =====================================
// STATE
// =====================================

globalThis.__TABFORGE_STREAM_STATE__ ??= {

sent:0,

errors:0,

lastPayload:null,

lastHash:null

};

const STATE=
globalThis.__TABFORGE_STREAM_STATE__;


// =====================================
// HELPERS
// =====================================

function sessionId(){

return crypto.randomUUID();

}


function blocked(url){

if(!url){
return true;
}

const blockedUrls=[

"chrome://",

"edge://",

"devtools://",

"chrome-extension://",

"about:"

];

return blockedUrls.some(

prefix=>

url.startsWith(
prefix
)

);

}



// =====================================
// FILTER
// =====================================

function sanitizeTabs(tabs){

if(
!Array.isArray(tabs)
){

return[];

}


return tabs

.filter(tab=>

tab &&
tab.id &&
tab.title &&
tab.url &&
!blocked(tab.url)

)

.map(tab=>({

id:
tab.id,

title:
tab.title,

url:
tab.url,

active:
tab.active ?? false,

windowId:
tab.windowId,

captureMode:
"native"

}));

}



// =====================================
// DEDUPE
// =====================================

function hash(payload){

return JSON.stringify(
payload
);

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
cleaned.length===0
){

console.warn(
"[TABSTREAM] no valid tabs"
);

return;

}


const payload={

sessionId:
sessionId(),

selected_tabs:
cleaned,

count:
cleaned.length,

timestamp:
Date.now()

};


const currentHash=

hash(payload.selected_tabs);


if(

STATE.lastHash===currentHash

){

console.log(
"[TABSTREAM] duplicate skipped"
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

chrome.runtime.lastError.message

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

"[BRIDGE SENT]",

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

function stats(){

console.table({

sent:
STATE.sent,

errors:
STATE.errors,

duplicateHash:
!!STATE.lastHash

});

}


function health(){

return{

loaded:true,

sent:
STATE.sent,

errors:
STATE.errors

};

}


function last(){

console.log(
STATE.lastPayload
);

}



// =====================================
// EXPORT
// =====================================

globalThis.sendTabsToTabForge=
sendTabsToTabForge;

globalThis.TabForgeTabStream={

sendTabsToTabForge,

stats,

health,

last

};


console.log(
"[TABSTREAM READY]"

);

})();