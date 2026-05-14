// TabForge Live Capture Engine

const activeCaptures = new Map();

async function initializeCaptureEngine() {

    console.log(
        "[TABFORGE] Live capture engine ready"
    );

    console.log(
        "[TABFORGE] Waiting for tab capture requests"
    );
}

async function startTabCapture(tabId) {

    try {

        console.log(
            `[CAPTURE] Requesting stream for tab ${tabId}`
        );

        const stream =
            await chrome.tabCapture.capture({

                audio: true,

                video: true,

                videoConstraints: {
                    mandatory: {
                        maxWidth: 1920,
                        maxHeight: 1080,
                        maxFrameRate: 60
                    }
                }
            });

        if (!stream) {

            console.error(
                `[CAPTURE] Failed for tab ${tabId}`
            );

            return;
        }

        console.log(
            `[CAPTURE] Stream active for tab ${tabId}`
        );

        const recorder =
            new MediaRecorder(
                stream,
                {
                    mimeType:
                        "video/webm;codecs=vp9,opus"
                }
            );

        recorder.ondataavailable =
            async (event) => {

            if (
                event.data &&
                event.data.size > 0
            ) {

                console.log(
                    `[CHUNK] ${tabId} -> ${event.data.size} bytes`
                );

                /*
                    Future:
                    Send chunk to Rust IPC
                */
            }
        };

        recorder.onstart = () => {

            console.log(
                `[RECORDER] Started ${tabId}`
            );
        };

        recorder.onstop = () => {

            console.log(
                `[RECORDER] Stopped ${tabId}`
            );
        };

        recorder.start(1000);

        activeCaptures.set(
            tabId,
            {
                stream,
                recorder
            }
        );

    } catch (error) {

        console.error(
            `[CAPTURE ERROR] ${tabId}`,
            error
        );
    }
}

function stopTabCapture(tabId) {

    const capture =
        activeCaptures.get(tabId);

    if (!capture) {

        console.warn(
            `[CAPTURE] No active session for ${tabId}`
        );

        return;
    }

    capture.recorder.stop();

    capture.stream
        .getTracks()
        .forEach(
            track => track.stop()
        );

    activeCaptures.delete(tabId);

    console.log(
        `[CAPTURE] Released ${tabId}`
    );
}

initializeCaptureEngine();

window.TabForgeCapture = {

    startTabCapture,

    stopTabCapture
};