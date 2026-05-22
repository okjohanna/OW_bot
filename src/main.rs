use dyncord::{Bot, Intents};
use dyncord::commands::Command;
use dyncord::commands::slash::context::SlashContext;

async fn handle_ping(ctx: SlashContext) {
    let user_id = ctx.event.author_id().unwrap();
    ctx.respond(format!("Pong, <@{}>! :wave:", user_id)).await.unwrap();
}

async fn handle_ow(ctx: SlashContext) {
    ctx.respond("Anybody up for some Overwatch? @everyone").await.unwrap();
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to read .env file");
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not set");

    let bot = Bot::new(())
        .intents(Intents::GUILD_MESSAGES)
        .intents(Intents::GUILD_MEMBERS)
        .intents(Intents::GUILD_MESSAGE_REACTIONS)
        .command(Command::slash("ping", handle_ping)
            .description("Ping"))
        .command(Command::slash("ow", handle_ow)
            .description("Invite everyone to play Overwatch"));

    bot.run(token).await.expect("Error starting the bot");
}