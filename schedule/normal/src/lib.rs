use schedule_flows::schedule_cron_job;
use slack_flows::send_message_to_channel;

#[no_mangle]
pub fn run() {
    schedule_cron_job(
        String::from("23 5 * * *"),
        String::from("cron_job_evoked"),
        |body| {
            send_message_to_channel(
                "reactorlocal",
                "random",
                String::from_utf8_lossy(&body).into_owned(),
            );
        },
    );
}
