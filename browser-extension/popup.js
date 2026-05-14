// TabForge - Popup Controller

const selectedTabs = new Map();

async function loadTabs() {
    try {
        const tabs = await chrome.tabs.query({});

        const container =
            document.getElementById("tabs");

        container.innerHTML = "";

        const validTabs = tabs.filter(tab =>
            tab.id &&
            tab.title &&
            tab.url &&
            !tab.url.startsWith("chrome://") &&
            !tab.url.startsWith("edge://")
        );

        if (validTabs.length === 0) {
            container.innerHTML =
                "<p>No available tabs found</p>";

            return;
        }

        validTabs.forEach(createTabItem);

    } catch (error) {

        console.error(
            "TabForge load error:",
            error
        );
    }
}

function createTabItem(tab) {

    const wrapper =
        document.createElement("div");

    wrapper.style.display = "flex";
    wrapper.style.alignItems = "center";
    wrapper.style.padding = "6px";
    wrapper.style.marginBottom = "8px";
    wrapper.style.borderBottom =
        "1px solid #ddd";

    const checkbox =
        document.createElement("input");

    checkbox.type = "checkbox";

    checkbox.addEventListener(
        "change",
        () => handleSelection(
            checkbox,
            tab
        )
    );

    const label =
        document.createElement("span");

    label.style.marginLeft = "8px";

    label.innerText =
        tab.title || "Untitled";

    wrapper.appendChild(checkbox);
    wrapper.appendChild(label);

    document
        .getElementById("tabs")
        .appendChild(wrapper);
}

function handleSelection(
    checkbox,
    tab
) {

    if (checkbox.checked) {

        selectedTabs.set(
            tab.id,
            {
                id: tab.id,
                title: tab.title,
                url: tab.url
            }
        );

    } else {

        selectedTabs.delete(
            tab.id
        );
    }

    const payload = {
        selectedTabs:
            Array.from(
                selectedTabs.values()
            ),

        totalSelected:
            selectedTabs.size,

        timestamp:
            Date.now()
    };

    sendTabsToTabForge(
        payload
    );
}

function sendTabsToTabForge(
    payload
) {

    console.log(
        "TABFORGE PAYLOAD:"
    );

    console.log(payload);
}

loadTabs();
