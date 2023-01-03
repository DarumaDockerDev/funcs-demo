use lambda_flows::{listen_to_request, message_from_request, send_response};
use slack_flows::send_message_to_channel;
use store_flows::{get, set};

#[no_mangle]
pub fn register() {
    listen_to_request();
}

#[no_mangle]
pub fn work() {
    let (_qry, body) = message_from_request();
    let count = match get("count") {
        Some(c) => c.as_i64().unwrap_or(0) + 1,
        None => 1,
    };
    set("count", serde_json::json!(count));

    if count % 2 == 0 {
        send_message_to_channel(
            "reactorlocal",
            "random",
            String::from_utf8_lossy(&body).into_owned(),
        );
    }
    let v = vec![1; 10000];
    send_response(v);
}
