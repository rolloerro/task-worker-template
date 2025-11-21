use std::time::Duration;
use tokio::time::sleep;

pub async fn run_worker() {
    loop {
        println!("Processing task...");
        sleep(Duration::from_secs(2)).await;
    }
}
