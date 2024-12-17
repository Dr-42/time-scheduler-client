#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    time_scheduler_client_lib::run().await
}
