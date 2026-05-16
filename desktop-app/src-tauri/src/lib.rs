mod capture;
mod runtime;

mod encoder;
mod ipc;
mod models;
mod recorder;
mod workers;

use runtime::bootstrap::
    initialize_runtime_services;

use ipc::bridge::{
    receive_tabs,
    TabData,
    TabPayload
};

use models::tab::
    BrowserTab;

use recorder::manager::
    RecorderManager;

use recorder::session::
    RecordingSession;


const APP_NAME:&str=
"TabForge";

const VERSION:&str=
"0.1.0";

const WIDTH:u32=1920;

const HEIGHT:u32=1080;

const FPS:u32=60;



#[tauri::command]

fn greet(
name:&str
)->String{

format!(
"Hello {}, welcome to {} {}",
name,
APP_NAME,
VERSION
)

}



fn create_browser_tab(

id:&str,

title:&str,

url:&str

)->BrowserTab{


BrowserTab{

id:
id.to_string(),

title:
title.to_string(),

url:
url.to_string(),

width:
WIDTH,

height:
HEIGHT,

fps:
FPS,

browser:
"Chrome"
.to_string(),

active:
true,

audio_available:
true,

stream_type:
"video/webm"
.to_string()

}

}



fn create_recording_session(

tab:BrowserTab

)->RecordingSession{

let filename=

tab.title

.to_lowercase()

.replace(
" ",
"_"
);


RecordingSession{

tab,

output_file:

format!(
"{}_1080p.mp4",
filename
),

fps:
FPS,

recording:
false,

audio_enabled:
true,

video_enabled:
true,

separate_audio:
true,

hardware_encoding:
true,

audio_bitrate:
320,

video_bitrate:
8000,

worker_id:
None,

stream_connected:
false

}

}



fn build_manager()

->RecorderManager{

let mut manager=

RecorderManager::new();


let tabs=vec![

create_browser_tab(
"tab_001",
"YouTube",
"https://youtube.com"
),

create_browser_tab(
"tab_002",
"Research",
"https://openai.com"
),

create_browser_tab(
"tab_003",
"Docs",
"https://docs.rs"
)

];


for tab in tabs{

manager.add_session(

create_recording_session(
tab
)

);

}


manager

}



fn simulate_extension_bridge(){

let payload=

TabPayload{

selected_tabs:

vec![

TabData{

id:
"tab_001"
.to_string(),

title:
"YouTube"
.to_string(),

url:
"https://youtube.com"
.to_string()

},

TabData{

id:
"tab_002"
.to_string(),

title:
"Research"
.to_string(),

url:
"https://openai.com"
.to_string()

},

TabData{

id:
"tab_003"
.to_string(),

title:
"Docs"
.to_string(),

url:
"https://docs.rs"
.to_string()

}

],

timestamp:
123456789

};


let sessions=

receive_tabs(
payload
);


println!();

println!(
"===== SESSION BUILDER ====="
);


for s in sessions{

println!(
"{} -> {}",

s.tab.title,

s.output_file
);

println!(
"Audio: {}",
s.audio_enabled
);

println!(
"Separate Audio: {}",
s.separate_audio
);

println!(
"HW Encode: {}",
s.hardware_encoding
);

println!(
"Worker: {:?}",
s.worker_id
);

println!(
"Stream: {}",
s.stream_connected
);

println!(
"----------------"
);

}

}



fn print_dashboard(

manager:
&RecorderManager

){

println!();

println!(
"========== {} {} ==========",
APP_NAME,
VERSION
);


for(

index,
session

)

in manager
.sessions
.iter()
.enumerate()

{

println!(
"[{}] {}",

index+1,

session.tab.title
);

println!(
"URL: {}",
session.tab.url
);

println!(
"Browser: {}",
session.tab.browser
);

println!(
"Resolution: {}",
session.tab.resolution()
);

println!(
"FPS: {}",
session.fps
);

println!(
"Audio: {}",
session.audio_enabled
);

println!(
"Video: {}",
session.video_enabled
);

println!(
"Output: {}",
session.output_file
);

println!(
"----------------"
);

}

}



#[cfg_attr(
mobile,
tauri::mobile_entry_point
)]

pub fn run(){

println!(
"[BOOT] Starting {} {}",
APP_NAME,
VERSION
);

initialize_runtime_services();

let manager=
build_manager();

print_dashboard(
&manager
);

simulate_extension_bridge();


tauri::Builder
::default()

.plugin(
tauri_plugin_opener
::init()
)

.invoke_handler(

tauri
::generate_handler![

greet

]

)

.run(

tauri
::generate_context!()

)

.expect(
"TabForge startup failed"
);

}