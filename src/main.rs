mod cli;
mod config;
mod modules;
#[macro_use]
extern crate serde_derive;
use config::Config;
#[macro_use]
extern crate pest_derive;

#[tokio::main]
async fn main() {
    Config::from_config(Config::path().await)
        .await
        .print()
        .await;
}
