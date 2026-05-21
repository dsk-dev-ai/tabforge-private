chrome.runtime.onInstalled.addListener(()=>{

console.log(
"[TABFORGE] Extension initialized"
);

});


chrome.runtime.onMessage.addListener(

async(

message,

sender,

sendResponse

)=>{


if(

message.action===

"START_CAPTURE"

){

try{

await window
.TabForgeCapture
.startTabCapture(

message.tabId

);


sendResponse({

success:true

});

}
catch(e){

sendResponse({

success:false,

error:e.toString()

});

}

}


if(

message.action===

"STOP_CAPTURE"

){

window
.TabForgeCapture
.stopTabCapture(

message.tabId

);

}


return true;

}

);