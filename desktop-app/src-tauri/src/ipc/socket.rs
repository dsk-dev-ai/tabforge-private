#[derive(Debug)]

pub struct SocketMessage {

    pub source: String,

    pub event: String,

    pub payload: String
}

pub fn initialize_socket() {

    println!(
        "[IPC] Socket initialized"
    );

    println!(
        "[IPC] Listening..."
    );
}

pub fn send_message(
    message: SocketMessage
) {

    println!(
        "[SOCKET EVENT]"
    );

    println!(
        "Source: {}",
        message.source
    );

    println!(
        "Event: {}",
        message.event
    );

    println!(
        "Payload: {}",
        message.payload
    );
}