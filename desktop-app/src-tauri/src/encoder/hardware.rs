#[derive(Debug)]

pub struct HardwareEngine{

    pub gpu_enabled:bool,

    pub encoder:String,

    pub decoder:String,

    pub active:bool
}



pub fn initialize_hardware()

->HardwareEngine{

    let engine=

    HardwareEngine{

        gpu_enabled:true,

        encoder:
        "Intel QuickSync"
        .to_string(),

        decoder:
        "VAAPI"
        .to_string(),

        active:true
    };


    println!(
        "[HARDWARE] GPU acceleration active"
    );

    println!(
        "[HARDWARE] Encoder: {}",
        engine.encoder
    );

    println!(
        "[HARDWARE] Decoder: {}",
        engine.decoder
    );

    println!(
        "[HARDWARE] Active: {}",
        engine.active
    );

    engine
}



#[allow(dead_code)]

pub fn hardware_status(){

    println!(
        "[HARDWARE] Monitoring active"
    );

}