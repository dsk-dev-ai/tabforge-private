#[derive(Debug, Clone)]
pub struct TabData {
    pub id: String,
    pub title: String,
    pub url: String,
}

#[derive(Debug)]
pub struct TabPayload {
    pub selected_tabs: Vec<TabData>,
    pub timestamp: u64,
}

pub fn receive_tabs(
    payload: TabPayload
) {

    println!();

    println!(
        "===== EXTENSION PAYLOAD ====="
    );

    for tab in payload.selected_tabs {

        println!(
            "[{}]",
            tab.id
        );

        println!(
            "Title: {}",
            tab.title
        );

        println!(
            "URL: {}",
            tab.url
        );

        println!(
            "----------------"
        );
    }

    println!(
        "Timestamp: {}",
        payload.timestamp
    );

    println!(
        "============================="
    );

    println!();
}