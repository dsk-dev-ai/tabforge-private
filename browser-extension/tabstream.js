function sendTabsToTabForge(tabIds){

    console.log(
        "Sending tabs:",
        tabIds
    );

    const payload = {

        selectedTabs: tabIds,

        timestamp:
            Date.now()

    };

    console.log(payload);

}

window.sendTabsToTabForge =
    sendTabsToTabForge;