#[derive(Clone, Debug)]
pub struct BrowserTab {
    pub id: String,
    pub title: String,
    pub url: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}