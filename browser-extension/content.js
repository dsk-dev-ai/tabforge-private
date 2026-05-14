console.log(
    "[TABFORGE] Content bridge active"
);

function getTabMetadata() {

    return {

        title:
            document.title,

        url:
            window.location.href,

        timestamp:
            Date.now(),

        visibility:
            document.visibilityState
    };
}

window.addEventListener(
    "load",
    () => {

        const metadata =
            getTabMetadata();

        console.log(
            "[TAB INFO]",
            metadata
        );

    }
);

document.addEventListener(
    "visibilitychange",
    ()=>{

        console.log(
            "[TAB VISIBILITY]",
            document.visibilityState
        );

    }
);