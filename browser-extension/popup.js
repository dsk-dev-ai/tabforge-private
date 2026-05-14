async function loadTabs() {

    const tabs = await chrome.tabs.query({});

    const container =
        document.getElementById("tabs");

    container.innerHTML="";

    tabs.forEach(tab=>{

        const item =
            document.createElement("div");

        item.innerText=
            tab.title;

        container.appendChild(item);

    });

}

loadTabs();