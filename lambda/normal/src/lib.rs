use lambda_flows::{listen_to_request, message_from_request, send_response};
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn register() {
    listen_to_request();
}

#[no_mangle]
pub fn work() {
    let (qry, body) = message_from_request();
    send_message_to_channel(
        "reactorlocal",
        "random",
        String::from_utf8_lossy(&body).into_owned(),
    );
    let v = vec![1; 10000];
    send_response(v);
}
