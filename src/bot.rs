use std::env;
use teloxide::prelude::*;

pub async fn notify(message: String) {
    let bot: Bot = Bot::new(env::var("TG_TOKEN").unwrap());

    let channel_id = ChatId(
        env::var("CHANNEL_ID")
            .unwrap()
            .parse::<i64>()
            .expect("Bad Channel ID, should be an int64"),
    );

    bot.send_message(channel_id, message)
        .parse_mode(teloxide::types::ParseMode::MarkdownV2)
        .await
        .unwrap();
}
