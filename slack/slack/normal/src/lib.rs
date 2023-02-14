use openai_flows::{create_completion, CompletionRequest};
use slack_flows::{listen_to_channel, send_message_to_channel};
// use twitter_flows::create_tweet;

#[no_mangle]
pub fn run() {
    listen_to_channel("reactorlocal", "t1", |sm| {
        let cr = CompletionRequest {
            prompt: sm.text,
            ..Default::default()
        };
        println!("-----");
        let r = create_completion("DarumaDocker", cr);
        println!("{:?}", r);
        r.iter().for_each(|c| {
            send_message_to_channel("reactorlocal-", "general", c.to_string());
        });

        /*
        let tweet = serde_json::json!({
            "text": sm.text
        });
        create_tweet(String::from("DarumaDockerDev"), tweet);
        */
    });
}
