let activeRecorders = new Map();

async function startTabCapture(tabId){

    try{

        const stream=

        await navigator.mediaDevices
        .getDisplayMedia({

            video:{
                preferCurrentTab:true
            },

            audio:true
        });


        const recorder=

        new MediaRecorder(

            stream,

            {
                mimeType:
                "video/webm"
            }

        );


        recorder.ondataavailable=

        async(event)=>{

            if(
                event.data.size<=0
            ){
                return;
            }


            const buffer=

            await event
            .data
            .arrayBuffer();


            console.log(
                "[CAPTURE]",
                tabId,
                buffer.byteLength
            );


            chrome.runtime
            .sendMessage({

                type:
                "STREAM_CHUNK",

                tabId:
                tabId,

                payload:
                Array.from(
                    new Uint8Array(
                        buffer
                    )
                ),

                timestamp:
                Date.now()

            });

        };


        recorder.start(
            1000
        );


        activeRecorders.set(

            tabId,

            recorder
        );


        console.log(
            "[CAPTURE STARTED]",
            tabId
        );

    }

    catch(error){

        console.error(
            error
        );

    }

}


function stopTabCapture(
    tabId
){

    const recorder=

    activeRecorders.get(
        tabId
    );


    if(
        !recorder
    ){
        return;
    }


    recorder.stop();

    activeRecorders.delete(
        tabId
    );


    console.log(
        "[CAPTURE STOPPED]",
        tabId
    );

}


window.startTabCapture=
startTabCapture;

window.stopTabCapture=
stopTabCapture;