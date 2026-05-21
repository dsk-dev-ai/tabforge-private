mod capture;
mod runtime;

mod encoder;
mod ipc;
mod models;
mod recorder;
mod workers;

// ========================================
// IMPORTS
// ========================================

use runtime::bootstrap::{
    initialize_runtime_services,
    runtime_alive
};

use models::tab::BrowserTab;

use recorder::manager::RecorderManager;

use recorder::session::RecordingSession;


// ========================================
// APP
// ========================================

const APP_NAME:&str="TabForge";

const VERSION:&str="1.0.0";

const DEFAULT_WIDTH:u32=1920;

const DEFAULT_HEIGHT:u32=1080;

const DEFAULT_FPS:u32=60;

const DEFAULT_AUDIO:u32=320;

const DEFAULT_VIDEO:u32=8000;


// ========================================
// COMMANDS
// ========================================

#[tauri::command]
fn greet(name:&str)->String{

    format!(
        "Hello {}, welcome to {} {}",
        name,
        APP_NAME,
        VERSION
    )

}


#[tauri::command]
fn runtime_status()->bool{

    runtime_alive()

}


#[tauri::command]
fn app_info()->String{

    format!(
        "{} {} | {}kbps audio | {}kbps video",
        APP_NAME,
        VERSION,
        DEFAULT_AUDIO,
        DEFAULT_VIDEO
    )

}


// ========================================
// TAB
// ========================================

fn browser_tab(

    id:&str,
    title:&str,
    url:&str

)->BrowserTab{

    BrowserTab{

        id:id.to_string(),

        title:title.to_string(),

        url:url.to_string(),

        width:DEFAULT_WIDTH,

        height:DEFAULT_HEIGHT,

        fps:DEFAULT_FPS

    }

}


// ========================================
// FILE OUTPUT
// ========================================

fn sanitize(v:&str)->String{

    v.to_lowercase()

    .replace(" ","_")

    .replace("/","_")

    .replace("\\","_")

    .replace(":","_")

    .replace("|","_")

    .replace("?","_")

    .replace("*","_")

}


fn output(tab:&BrowserTab)->String{

    format!(
        "recordings/{}_{}p.mp4",
        sanitize(&tab.title),
        tab.height
    )

}


// ========================================
// SESSION
// ========================================

fn create_session(
    tab:BrowserTab
)->RecordingSession{

    // FIX: borrow before move
    let output_file=
        output(&tab);

    let fps=
        tab.fps;

    RecordingSession{

        output_file,

        fps,

        tab,

        recording:false,

        audio_enabled:true,

        video_enabled:true,

        hardware_encoding:true,

        separate_audio:true

    }

}



// ========================================
// MANAGER
// ========================================

fn create_manager()->RecorderManager{

    let mut manager=
        RecorderManager::new();


    manager.sessions.push(

        create_session(

            browser_tab(

                "runtime",

                "TabForge Runtime",

                "http://localhost"

            )

        )

    );


    manager

}



// ========================================
// DASHBOARD
// ========================================

fn dashboard(
    manager:&RecorderManager
){

    println!();

    println!(
        "========== {} ==========",
        APP_NAME
    );

    println!(
        "Version {}",
        VERSION
    );

    println!(
        "Sessions {}",
        manager.sessions.len()
    );

    println!(
        "Audio {}kbps",
        DEFAULT_AUDIO
    );

    println!(
        "Video {}kbps",
        DEFAULT_VIDEO
    );

    println!(
        "Runtime {}",
        if runtime_alive(){

            "ONLINE"

        }else{

            "OFFLINE"

        }
    );

    println!(
        "========================"
    );

    println!();

}



// ========================================
// RUNTIME
// ========================================

fn initialize_runtime(){

    println!(
        "[BOOT] Starting {} {}",
        APP_NAME,
        VERSION
    );

    initialize_runtime_services();

}



// ========================================
// ENTRY
// ========================================

#[cfg_attr(mobile, tauri::mobile_entry_point)]

pub fn run(){

    initialize_runtime();

    let manager=
        create_manager();

    dashboard(
        &manager
    );


    tauri::Builder::default()

    .plugin(
        tauri_plugin_opener::init()
    )

    .invoke_handler(

        tauri::generate_handler![

            greet,

            runtime_status,

            app_info

        ]

    )

    .run(

        tauri::generate_context!()

    )

    .expect(

        "TabForge startup failed"

    );

}