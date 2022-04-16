use dotenv::dotenv;
use std::env;

mod event_handler;
mod commands;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    dotenv().ok();

    let token = env::var("TOKEN").expect("No token found");

    let mut client = serenity::Client::builder(token)
        .event_handler(event_handler::Handler)
        .framework(commands::load_commands())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Error when start client: {:?}", why)
    }
}


