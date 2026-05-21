// ====================================================
// TabForge Tab Stream Bridge v2.0
// Popup → Runtime → Background
// Production Transport Layer
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

lastPayload:null

};

const STATE=
globalThis.__TABFORGE_STREAM_STATE__;



// =====================================
// TAB FILTER
// =====================================

function sanitizeTabs(tabs){

if(
!Array.isArray(tabs)
){

return [];
}


return tabs

.filter(tab=>

tab &&
tab.id &&
tab.title &&
tab.url &&

!tab.url.startsWith(
"chrome://"
) &&

!tab.url.startsWith(
"edge://"
) &&

!tab.url.startsWith(
"devtools://"
)

)

.map(tab=>({

id:
tab.id,

title:
tab.title,

url:
tab.url,

active:
tab.active,

windowId:
tab.windowId

}));

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

selected_tabs:
cleaned,

count:
cleaned.length,

timestamp:
Date.now()

};


STATE.lastPayload=
payload;

STATE.sent++;


chrome.runtime.sendMessage(

{

action:
"TAB_PAYLOAD",

payload

},

(response)=>{

if(

chrome.runtime.lastError

){

STATE.errors++;

console.error(

"[BRIDGE ERROR]",

chrome.runtime.lastError

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
STATE.errors

});

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

last

};


console.log(
"[TABSTREAM READY]"
);

})();