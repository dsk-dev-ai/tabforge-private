// ====================================================
// TabForge Popup Runtime v4.0
// Native Capture Dashboard
// Phase A
// ====================================================

(()=>{

console.log(
"[TABFORGE POPUP] Boot"
);


// =====================================
// STATE
// =====================================

globalThis.__TABFORGE_POPUP__ ??= {

selected:new Set(),

poller:null,

tabs:[]

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

"about:"

]

.some(

v=>url.startsWith(v)

);

}


function button(text){

const b=
document.createElement(
"button"
);

b.innerText=text;

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
// RENDER
// =====================================

function render(tabs){

const container=

document.getElementById(
"tabs"
);

container.innerHTML="";


tabs.forEach(
createCard
);

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



const native=

document.createElement(
"span"
);

native.innerText=
"Native";

native.style.marginLeft=
"auto";

native.style.fontSize=
"10px";

native.style.opacity=
".6";



top.appendChild(icon);

top.appendChild(title);

top.appendChild(native);



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

const stop=

button(
"Stop"
);


if(
STATE.selected.has(tab.id)
){

start.disabled=true;

}


start.onclick=

()=>startCapture(

tab,
start

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
// START
// =====================================

function startCapture(

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


buttonEl.disabled=true;


STATE.selected.add(
tab.id
);


update();


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

STATE.selected.delete(
tab.id
);

buttonEl.disabled=false;

showError(

response?.error

);

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

buttonEl.disabled=false;


update();


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

function update(){

document
.getElementById(
"status"
)

.innerText=

`Recording ${STATE.selected.size}`;

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
// RUNTIME
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


document
.getElementById(
"runtime"
)

.innerText=

`Sessions ${response.active?.length || 0}`;


}

);

},3000);

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

`Runtime Error`;

}



// =====================================
// EXIT
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