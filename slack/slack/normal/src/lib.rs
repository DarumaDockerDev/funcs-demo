use slack_flows::{
    listen_to_channel, message_from_channel, revoke_listeners, send_message_to_channel,
};

#[no_mangle]
pub fn prepare() {
    revoke_listeners();
    /*
    if let Some(sm) = listen_to_channel("reactorlocal", "random") {
        send_message_to_channel("reactorlocal", "general", sm.text);
    }
    */
}

#[no_mangle]
pub fn work() {
    if let Some(sm) = message_from_channel() {
        send_message_to_channel("reactorlocal", "random", sm.text + "---");
    }
}
