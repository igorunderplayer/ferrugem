use serenity::{
    client::Context,
    model::channel::Message
};

use serenity::framework::standard::{
    macros::{group, command},
    CommandResult
};

use simple_process_stats::ProcessStats;


#[group]
#[commands(avatar, botinfo, embed, ping)]
pub struct Info;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong").await?;
    Ok(())
}

#[command]
async fn botinfo(ctx: &Context, msg: &Message) -> CommandResult {
    let process_stats = ProcessStats::get().await.expect("could not get stats for running process");
    let process_uptime_minutes = process_stats.cpu_time_user.as_secs() / 60;
    let memory = process_stats.memory_usage_bytes / 1024 / 1024;
    msg.channel_id.send_message(&ctx.http, |m|
        m.embed(|e|
                e
                    .title("Minhas infos.")
                    .color(0x5865F2)
                    .description(
                            format!(
                                "Memoria usada/alocada (deve ser): {}MiB
                                Uptime(processo): {} minutos
                                ", memory, process_uptime_minutes)
                        )
            )
    ).await?;
    Ok(())
}


#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let user = match msg.mentions.get(0) {
        Some(user) => user,
        None => &msg.author
    };

    let avatar_url = match user.avatar.to_owned() {
        Some(avatar) => {
            let avatar_format = if avatar.starts_with("a_") { "gif" } else { "png" };
            let formated = format!("https://cdn.discordapp.com/avatars/{}/{}.{}?size=4096", user.id, avatar, avatar_format);
            formated
        },

        None => user.default_avatar_url()
    };

    msg.channel_id.send_message(&ctx.http, |m|
        m.embed(|e|
            e
                .title(format!("Avatar de {}", user.name))
                .color(0xd3d3d3)
                .image(avatar_url)
        )).await?;

    Ok(())
}

#[command]
async fn embed(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e|
            e.title("EMBED PICA").description(":flushed: isso eh so uma embed de testes, n me estranhe")
        )}).await?;

    Ok(())
}
