// ====================================================
// TabForge Popup Runtime v3.0
// Production Dashboard UI
// ====================================================

(() => {

console.log(
"[TABFORGE POPUP] Boot"
);


globalThis.__TABFORGE_POPUP__ ??= {

selected:new Set(),

poller:null

};

const STATE=
globalThis.__TABFORGE_POPUP__;



// =====================================
// LOAD
// =====================================

async function loadTabs(){

try{

const tabs=
await chrome.tabs.query({});


const container=
document.getElementById(
"tabs"
);

container.innerHTML="";


const valid=

tabs.filter(

tab=>

tab.id &&
tab.url &&
tab.title &&

!tab.url.startsWith(
"chrome://"
) &&

!tab.url.startsWith(
"edge://"
) &&

!tab.url.startsWith(
"devtools://"
)

);


document
.getElementById(
"count"
)

.innerText=

`${valid.length} tabs`;


valid.forEach(
createCard
);

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

function createCard(tab){

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



const icon=
document.createElement(
"img"
);

icon.width=16;

icon.height=16;

icon.src=

tab.favIconUrl ||

"icons/icon16.png";

icon.style.marginRight=
"6px";



const title=
document.createElement(
"div"
);

title.className=
"title";

title.innerText=

tab.title.substring(
0,
50
);



top.appendChild(
icon
);

top.appendChild(
title
);



const url=
document.createElement(
"div"
);

url.className=
"url";

url.innerText=
tab.url;



const controls=
document.createElement(
"div"
);

controls.className=
"controls";



const start=
button(
"Start"
);

start.onclick=
()=>startCapture(
tab,
start
);



const stop=
button(
"Stop"
);

stop.onclick=
()=>stopCapture(
tab,
start
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
// BUTTON
// =====================================

function button(text){

const b=
document.createElement(
"button"
);

b.innerText=text;

return b;

}



// =====================================
// START
// =====================================

async function startCapture(

tab,
buttonEl

){

if(
STATE.selected.has(
tab.id
)
){

return;

}


buttonEl.disabled=
true;


STATE.selected.add(
tab.id
);


updateStatus();


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

showError(
response?.error
);

buttonEl.disabled=
false;

return;

}


console.log(
response
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

STATE.selected.delete(
tab.id
);

buttonEl.disabled=
false;


updateStatus();


chrome.runtime.sendMessage({

action:
"STOP_CAPTURE",

tabId:
tab.id

});

}



// =====================================
// STATUS
// =====================================

function updateStatus(){

document
.getElementById(
"status"
)

.innerText=

`Recording ${STATE.selected.size}`;

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

`Error`;

}



// =====================================
// STATS
// =====================================

function runtimePoll(){

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


document
.getElementById(
"runtime"
)

.innerText=

`Sessions ${response.active.length}`;

}

);

},5000);

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



runtimePoll();

loadTabs();

})();