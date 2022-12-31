use schedule_flows::{listen_to_request, message_from_request};
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn register() {
    listen_to_request(String::from("* * *"), String::from("ababa"));
}

#[no_mangle]
pub fn work() {
    let body = message_from_request();
    send_message_to_channel(
        "reactorlocal",
        "random",
        String::from_utf8_lossy(&body).into_owned(),
    );
}
