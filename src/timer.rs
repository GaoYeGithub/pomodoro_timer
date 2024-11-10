use tokio::time::{self, Duration};
use indicatif::ProgressBar;
use notify_rust::Notification;

pub async fn run_timer(minutes: u64, message: &str, pb: &ProgressBar) {
    let total_seconds = minutes * 60;
    pb.set_length(total_seconds);

    for _ in 0..total_seconds {
        pb.inc(1);
        time::sleep(Duration::from_secs(1)).await;
    }

    pb.finish_with_message(message.to_string());
    
    if let Err(e) = Notification::new()
        .summary("Pomodoro Timer")
        .body(message)
        .show() {
        eprintln!("Failed to send notification: {}", e);
    }
}