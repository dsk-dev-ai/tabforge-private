// ====================================================
// TabForge Popup Runtime v6.0
// Native Session Dashboard
// Phase B2 Stable
// ====================================================

(()=>{

console.log(
"[TABFORGE POPUP] PhaseB2 Stable Boot"
);


// =====================================
// STATE
// =====================================

globalThis.__TABFORGE_POPUP__ ??= {

selected:new Set(),

tabs:[],

runtime:null,

poller:null

};

const STATE=
globalThis.__TABFORGE_POPUP__;



// =====================================
// HELPERS
// =====================================

function blocked(url){

if(!url){

return true;

}

return [

"chrome://",
"edge://",
"devtools://",
"chrome-extension://",
"about:",
"view-source:"

]

.some(

v=>url.startsWith(v)

);

}


function createButton(text){

const b=

document.createElement(
"button"
);

b.innerText=
text;

return b;

}



// =====================================
// LOAD
// =====================================

async function loadTabs(){

try{

const tabs=

await chrome.tabs.query({});


STATE.tabs=

tabs.filter(

tab=>

tab.id &&
tab.url &&
tab.title &&
!blocked(tab.url)

);


render(
STATE.tabs
);


document
.getElementById(
"count"
)

.innerText=

`${STATE.tabs.length} tabs`;

}
catch(error){

showError(
error
);

}

}



// =====================================
// CARD
// =====================================

function card(tab){

const root=

document.createElement(
"div"
);

root.className=
"tabCard";



const top=

document.createElement(
"div"
);

top.style.display=
"flex";

top.style.alignItems=
"center";



const icon=

document.createElement(
"img"
);

icon.width=16;

icon.height=16;

icon.style.marginRight=
"8px";

icon.src=

tab.favIconUrl ||

"icons/icon16.png";



const title=

document.createElement(
"div"
);

title.className=
"title";

title.innerText=

tab.title.slice(
0,
50
);



const badge=

document.createElement(
"span"
);

badge.style.marginLeft=
"auto";

badge.style.fontSize=
"10px";


if(
STATE.selected.has(tab.id)
){

badge.innerText=
"● LIVE";

badge.style.color=
"#4ade80";

}
else{

badge.innerText=
"NATIVE";

badge.style.opacity=
".6";

}



top.appendChild(
icon
);

top.appendChild(
title
);

top.appendChild(
badge
);



// URL

const url=

document.createElement(
"div"
);

url.className=
"url";

url.innerText=
tab.url;



const sid=

document.createElement(
"div"
);

sid.className=
"url";

sid.innerText=

`tab:${tab.id}`;



// controls

const controls=

document.createElement(
"div"
);

controls.className=
"controls";


const start=

createButton(
"Start"
);

const stop=

createButton(
"Stop"
);



if(
STATE.selected.has(tab.id)
){

start.disabled=true;

stop.disabled=false;

}
else{

start.disabled=false;

stop.disabled=true;

}



start.onclick=

()=>startCapture(
tab,
start
);


stop.onclick=

()=>stopCapture(
tab,
stop
);



controls.appendChild(
start
);

controls.appendChild(
stop
);



root.appendChild(
top
);

root.appendChild(
url
);

root.appendChild(
sid
);

root.appendChild(
controls
);


document
.getElementById(
"tabs"
)

.appendChild(
root
);

}



// =====================================
// RENDER
// =====================================

function render(tabs){

const container=

document.getElementById(
"tabs"
);

container.innerHTML="";


tabs.forEach(
card
);

}



// =====================================
// START
// =====================================

function startCapture(

tab,
buttonEl

){

if(
STATE.selected.has(tab.id)
){

return;

}


buttonEl.disabled=true;


chrome.runtime.sendMessage(

{

action:
"START_CAPTURE",

tabId:
tab.id

},

response=>{


if(
!response?.success
){

buttonEl.disabled=
false;

showError(
response?.error
);

return;

}


STATE.selected.add(
tab.id
);


update();

render(
STATE.tabs
);


console.log(

"[STARTED]",

tab.id

);

}

);

}



// =====================================
// STOP
// =====================================

function stopCapture(

tab,
buttonEl

){

chrome.runtime.sendMessage(

{

action:
"STOP_CAPTURE",

tabId:
tab.id

},

response=>{


if(
!response?.success
){

showError(
response?.error
);

return;

}


STATE.selected.delete(
tab.id
);


buttonEl.disabled=
false;


update();

render(
STATE.tabs
);


console.log(

"[STOPPED]",

tab.id

);

}

);

}



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


const filtered=

STATE.tabs.filter(

tab=>

tab.title
.toLowerCase()
.includes(q)

||

tab.url
.toLowerCase()
.includes(q)

);


render(
filtered
);

}

);



// =====================================
// UPDATE
// =====================================

function update(){

document
.getElementById(
"status"
)

.innerText=

`Recording ${STATE.selected.size}`;

}



// =====================================
// POLL
// =====================================

function poll(){

STATE.poller=

setInterval(()=>{

chrome.runtime.sendMessage(

{

action:
"RUNTIME_STATS"

},

response=>{


if(
!response
){

return;

}


STATE.runtime=
response;


STATE.selected=

new Set(
response.active || []
);



document
.getElementById(
"runtime"
)

.innerHTML=

`

Sessions ${
response.active?.length || 0
}

<br>

Starts ${
response.metrics?.starts || 0
}

<br>

Streams ${
response.metrics?.nativeStreams || 0
}

<br>

Errors ${
response.metrics?.errors || 0
}

`;



update();

render(
STATE.tabs
);

}

);

},2000);

}



// =====================================
// ERROR
// =====================================

function showError(error){

console.error(
error
);


document
.getElementById(
"runtime"
)

.innerText=

"Runtime Error";

}



// =====================================
// CLEANUP
// =====================================

window.addEventListener(

"unload",

()=>{

clearInterval(
STATE.poller
);

}

);



poll();

loadTabs();

})();