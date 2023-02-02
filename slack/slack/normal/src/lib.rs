use slack_flows::{listen_to_channel, send_message_to_channel};
use twitter_flows::create_tweet;

#[no_mangle]
pub fn run() {
    listen_to_channel("reactorlocal", "t1", |sm| {
        send_message_to_channel("reactorlocal", "general", format!("Hello, {}", sm.text));
        let tweet = serde_json::json!({
            "text": sm.text
        });
        create_tweet(tweet);
    });
}
