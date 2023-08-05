use anyhow::Result;
use async_trait::async_trait;
use teloxide::{adaptors::DefaultParseMode, prelude::*, utils::command::BotCommands};

use crate::Notifier;

#[derive(Debug, Clone)]
pub struct Telegram(DefaultParseMode<Bot>);

impl Telegram {
    pub fn new(token: String) -> Self {
        let bot = teloxide::Bot::new(token).parse_mode(teloxide::types::ParseMode::Html);
        Self(bot)
    }

    pub async fn send_message(&self, chat_id: i64, text: &str) -> Result<()> {
        self.0.send_message(ChatId(chat_id), text).await?;

        Ok(())
    }
}

#[async_trait]
impl Notifier for Telegram {
    async fn send_notification(&self, user_key: &str, message: &str) -> Result<(), anyhow::Error> {
        let chat_id = user_key.parse()?;

        self.send_message(chat_id, message).await?;

        Ok(())
    }

    async fn send_notification_with_title(
        &self,
        user_key: &str,
        title: &str,
        message: &str,
    ) -> Result<(), anyhow::Error> {
        let message = format!("<b>{title}</b>\n{message}");

        let chat_id = user_key.parse()?;

        self.send_message(chat_id, &message).await?;

        Ok(())
    }

    async fn send_notification_with_title_and_url(
        &self,
        user_key: &str,
        title: &str,
        message: &str,
        url: &str,
        url_title: &str,
    ) -> Result<(), anyhow::Error> {
        let message = format!("<b>{title}</b>\n{message}\n<a href=\"{url}\">{url_title}</a>");

        let chat_id = user_key.parse()?;

        self.send_message(chat_id, &message).await?;

        Ok(())
    }
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum TelegramCommand {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "notify me when there is an update")]
    NotifyMe,
}

async fn answer(
    bot: DefaultParseMode<Bot>,
    message: Message,
    command: TelegramCommand,
) -> ResponseResult<()> {
    match command {
        TelegramCommand::Help => {
            bot.send_message(message.chat.id, TelegramCommand::descriptions().to_string())
                .await?
        }
        TelegramCommand::NotifyMe => {
            bot.send_message(
                message.chat.id,
                format!(
                    "Put the following chat id on tanoshi profile settings: {}",
                    message.chat.id
                ),
            )
            .await?
        }
    };

    Ok(())
}

pub async fn run(bot: Telegram) {
    info!("start telegram bot");
    TelegramCommand::repl(bot.0, answer).await;
}
