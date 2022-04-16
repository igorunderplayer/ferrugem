use serenity::client::EventHandler;

use serenity::{
    async_trait,
    client::Context,
    model::{channel::Message, gateway::Ready}
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready (&self, _ctx: Context, ready: Ready) {
        println!("Logged as {}", ready.user.name);
    }

    async fn message (&self, ctx: Context, message: Message) {
        if message.author.bot { return; }
        if message.content == "sexo!" {
            message.channel_id.say(&ctx.http, "eca sexo").await.expect("Failed to send message");
        }
    }
}
