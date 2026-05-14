#[derive(Debug)]
pub struct TabPayload {
    pub selected_tabs: Vec<String>,
    pub timestamp: u64,
}

pub fn receive_tabs(
    payload: TabPayload
) {
    println!(
        "Incoming tabs: {:?}",
        payload.selected_tabs
    );

    println!(
        "Timestamp: {}",
        payload.timestamp
    );
}
