use serenity::framework::standard::StandardFramework;

mod info;

pub fn load_commands() -> StandardFramework {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("f!"))
        .group(&self::info::INFO_GROUP);

    return framework;
}
