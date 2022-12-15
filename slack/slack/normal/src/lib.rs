use slack_flows::{get_event, message_from_channel, send_message_to_channel};

#[no_mangle]
pub fn prepare() {
    if let Some(sm) = message_from_channel("reactorlocal", "random") {
        send_message_to_channel("reactorlocal", "general", sm.text);
    }
}

#[no_mangle]
pub fn work() {
    if let Some(sm) = get_event() {
        send_message_to_channel("reactorlocal", "random", sm.text + "---");
        send_message_to_channel("reactorlocal", "random", String::from("^_%"));
    }
}
