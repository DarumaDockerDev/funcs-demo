use lambda_flows::{listen_to_request, message_from_request, send_response};
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn prepare() {
    listen_to_request();
}

#[no_mangle]
pub fn work() {
    let (qry, body) = message_from_request();
    send_message_to_channel("reactorlocal", "random", String::from("new message"));
    send_response("abc".as_bytes().to_vec());
}
