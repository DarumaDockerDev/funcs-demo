use slack_flows::{channel_msg_received, send_message_to_channel};

#[no_mangle]
pub fn run() {
    if let Some(sm) = channel_msg_received("reactorlocal", "t1") {
        send_message_to_channel("reactorlocal", "general", format!("Hello, {}", sm.text));
        send_message_to_channel("reactorlocal", "t1", String::from("new message"));
    }
}
