#[derive(
    Clone,
    Debug
)]

pub struct BrowserTab{

    pub id:String,

    pub title:String,

    pub url:String,

    pub width:u32,

    pub height:u32,

    pub fps:u32,

    pub browser:String,

    pub active:bool,

    pub audio_available:bool,

    pub stream_type:String
}


impl BrowserTab{

    #[allow(dead_code)]

    pub fn summary(
        &self
    )->String{

        format!(

            "{} | {}x{} | {} FPS | {}",

            self.title,

            self.width,

            self.height,

            self.fps,

            self.browser
        )

    }


    #[allow(dead_code)]

    pub fn resolution(
        &self
    )->String{

        format!(
            "{}x{}",

            self.width,

            self.height
        )

    }


    #[allow(dead_code)]

    pub fn stream_label(
        &self
    )->String{

        format!(

            "{} ({})",

            self.stream_type,

            self.id
        )

    }
}