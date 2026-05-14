chrome.runtime.onInstalled.addListener(() => {
    console.log("TabForge Bridge initialized");
});

chrome.tabs.onActivated.addListener(async (activeInfo) => {

    const tab = await chrome.tabs.get(
        activeInfo.tabId
    );

    console.log("Active Tab:");

    console.log({
        id: tab.id,
        title: tab.title,
        url: tab.url
    });

});