async function initializeCapture() {

    console.log(
        "[TABFORGE] Capture engine online"
    );

    console.log(
        "[TABFORGE] Waiting for selected tabs"
    );
}

async function captureTab(
    tabId
){

    console.log(
        `[CAPTURE] Starting tab ${tabId}`
    );

    /*
      Future:

      chrome.tabCapture.capture({
          audio:true,
          video:true
      })

    */
}

initializeCapture();