// ====================================================
// TabForge Popup Runtime v7
// MV3 Stable Dashboard
// ====================================================

(() => {

if(window.__TABFORGE_POPUP__LOADED__){

console.log("[POPUP] already loaded");

return;

}

window.__TABFORGE_POPUP__LOADED__=true;

console.log("[POPUP] boot");


// =====================================
// STATE
// =====================================

const STATE={

selected:new Set(),

tabs:[],

runtime:{},

poller:null

};


// =====================================
// HELPERS
// =====================================

function blocked(url){

if(!url)return true;

return [

"chrome://",
"edge://",
"devtools://",
"chrome-extension://",
"about:",
"view-source:"

].some(v=>url.startsWith(v));

}


function send(message){

return new Promise(resolve=>{

try{

chrome.runtime.sendMessage(

message,

response=>{

if(

chrome.runtime.lastError

){

console.warn(

"[SEND ERROR]",

chrome.runtime.lastError.message

);

resolve(null);

return;

}

resolve(response);

}

);

}
catch(e){

console.error(e);

resolve(null);

}

});

}


function btn(text,className){

const b=document.createElement("button");

b.innerText=text;

b.className=className;

return b;

}


// =====================================
// LOAD TABS
// =====================================

async function loadTabs(){

const tabs=

await chrome.tabs.query({});

STATE.tabs=tabs.filter(

t=>

t.id &&

t.url &&

!blocked(t.url)

);

render();

updateStats();

}


// =====================================
// START
// =====================================

async function startCapture(tabId){

const result=

await send({

action:"START_CAPTURE",

tabId

});


if(

!result?.success

){

showError(

result?.error ||

"start failed"

);

return;

}

STATE.selected.add(tabId);

render();

updateStats();

console.log(

"[START]",

tabId

);

}



// =====================================
// STOP
// =====================================

async function stopCapture(tabId){

const result=

await send({

action:"STOP_CAPTURE",

tabId

});


if(

!result?.success

){

showError(

result?.error ||

"stop failed"

);

return;

}

STATE.selected.delete(tabId);

render();

updateStats();

console.log(

"[STOP]",

tabId

);

}



// =====================================
// CARD
// =====================================

function card(tab){

const root=

document.createElement("div");

root.className="tabCard";


const title=

document.createElement("div");

title.className="title";

title.innerText=

tab.title.slice(0,60);


const url=

document.createElement("div");

url.className="url";

url.innerText=tab.url;


const controls=

document.createElement("div");

controls.className="controls";


const live=

STATE.selected.has(tab.id);


const start=

btn("Start","start");

const stop=

btn("Stop","stop");


start.disabled=live;

stop.disabled=!live;


start.addEventListener(

"click",

()=>startCapture(tab.id)

);


stop.addEventListener(

"click",

()=>stopCapture(tab.id)

);


controls.append(

start,

stop

);


root.append(

title,

url,

controls

);


return root;

}



// =====================================
// RENDER
// =====================================

function render(){

const tabs=

document.getElementById(

"tabs"

);

tabs.innerHTML="";


STATE.tabs.forEach(

tab=>{

tabs.appendChild(

card(tab)

);

}

);

}



// =====================================
// STATS
// =====================================

function updateStats(){

document
.getElementById(
"count"
)

.innerText=

STATE.tabs.length;


document
.getElementById(
"status"
)

.innerText=

STATE.selected.size;


document
.getElementById(
"runtimeCount"
)

.innerText=

STATE.runtime
.active?.length || 0;

}



// =====================================
// POLL
// =====================================

async function poll(){

const result=

await send({

action:
"RUNTIME_STATS"

});


if(!result){

return;

}


STATE.runtime=result;


STATE.selected=

new Set(

result.active||[]

);


updateStats();

render();

}


STATE.poller=

setInterval(

poll,

3000

);


// =====================================
// SEARCH
// =====================================

document
.getElementById(
"search"
)

.addEventListener(

"input",

e=>{

const q=

e.target.value
.toLowerCase();


STATE.tabs=

STATE.tabs.filter(

t=>

t.title
.toLowerCase()
.includes(q)

||

t.url
.toLowerCase()
.includes(q)

);


render();

}

);


// =====================================
// REFRESH
// =====================================

document
.getElementById(
"refreshBtn"
)

?.addEventListener(

"click",

()=>{

loadTabs();

poll();

}

);


document
.getElementById(
"stopAll"
)

?.addEventListener(

"click",

()=>{

[...STATE.selected]

.forEach(

stopCapture

);

}

);


// =====================================
// ERROR
// =====================================

function showError(error){

console.error(

"[POPUP]",

error

);


document
.getElementById(
"runtime"
)

.innerText=

String(error);

}


window.addEventListener(

"unload",

()=>{

clearInterval(

STATE.poller

);

}

);


loadTabs();

poll();

})();