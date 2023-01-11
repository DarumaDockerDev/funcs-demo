use schedule_flows::cron_job_evoked;
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn run() {
    if let Some(body) = cron_job_evoked(String::from("50 8 * * *"), String::from("cron_job_evoked"))
    {
        send_message_to_channel(
            "reactorlocal",
            "random",
            String::from_utf8_lossy(&body).into_owned(),
        );
    }
}
