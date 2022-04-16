use serenity::{
    client::Context,
    model::channel::Message,
};

use serenity::framework::standard::{
    macros::{group, command},
    CommandResult
};

use simple_process_stats::ProcessStats;


#[group]
#[commands(ping)]
pub struct Info;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let process_stats = ProcessStats::get().await.expect("could not get stats for running process");
    msg.reply(ctx, "pong").await?;
    msg.reply(ctx, format!("Memory usage {:?}MiB", process_stats.memory_usage_bytes / 1024 / 1024 )).await?;
    Ok(())
}
