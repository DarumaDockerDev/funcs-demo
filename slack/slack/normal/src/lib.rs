use slack_flows::{listen_to_channel, send_message_to_channel};

#[no_mangle]
pub fn run() {
    listen_to_channel("reactorlocal", "t1", |sm| {
        send_message_to_channel("reactorlocal", "general", format!("Hello, {}", sm.text));
        send_message_to_channel("reactorlocal", "t1", String::from("new message"));
    });
}
