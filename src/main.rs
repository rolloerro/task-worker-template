mod worker;
mod tasks;

#[tokio::main]
async fn main() {
    println!("Task Worker started!");
    worker::run_worker().await;
}
