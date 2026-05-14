let selectedTabs = [];

async function loadTabs() {
    const tabs = await chrome.tabs.query({});

    const container =
        document.getElementById("tabs");

    container.innerHTML = "";

    tabs.forEach((tab) => {

        const wrapper =
            document.createElement("div");

        wrapper.style.marginBottom = "8px";

        const checkbox =
            document.createElement("input");

        checkbox.type = "checkbox";

        checkbox.onchange = () => {

            if (checkbox.checked) {

                selectedTabs.push(tab.id);

            } else {

                selectedTabs =
                    selectedTabs.filter(
                        id => id !== tab.id
                    );
            }

            console.log(
                "Selected tabs:",
                selectedTabs
            );

            sendTabsToTabForge(
                selectedTabs
            );
        };

        const label =
            document.createElement("span");

        label.innerText =
            " " + (tab.title || "Untitled");

        wrapper.appendChild(
            checkbox
        );

        wrapper.appendChild(
            label
        );

        container.appendChild(
            wrapper
        );
    });
}

function sendTabsToTabForge(tabIds) {

    const payload = {
        selectedTabs: tabIds,
        timestamp: Date.now()
    };

    console.log(
        "Sending:",
        payload
    );
}

loadTabs();