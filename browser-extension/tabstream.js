function sendTabsToTabForge(

tabs

){

console.log(
"[BRIDGE]"
);

console.log(
tabs
);


const payload={

selected_tabs:

tabs.map(

tab=>({

id:
tab.id,

title:
tab.title,

url:
tab.url

})

),

timestamp:
Date.now()

};


console.log(
payload
);


chrome.runtime.sendMessage({

action:
"TAB_PAYLOAD",

payload

});

}


window.sendTabsToTabForge=

sendTabsToTabForge;