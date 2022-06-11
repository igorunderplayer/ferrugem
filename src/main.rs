use dotenv::dotenv;
use std::env;

use serenity::framework::StandardFramework;

mod commands;
mod event_handler;

use commands::info::*;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let token = env::var("TOKEN").expect("No token found");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("r!"))
        .group(&INFO_GROUP);

    let mut client = serenity::Client::builder(token)
        .event_handler(event_handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Error when start client: {:?}", why)
    }
}


