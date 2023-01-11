use lambda_flows::{request_received, send_response};
use slack_flows::{send_message_to_channel, upload_file};
use store_flows::{get, set};

#[no_mangle]
pub fn run() {
    upload_file(
        "reactorlocal",
        "t1",
        "arch.jpg",
        "jpg",
        include_bytes!("./arch.jpg").to_vec(),
    );

    if let Some((_qry, body)) = request_received() {
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
        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            "ok".as_bytes().to_vec(),
        );
    }
}
