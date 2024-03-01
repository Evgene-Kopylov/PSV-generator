use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Debug, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Этот текст.")]
    Help,
    #[command(description = "Кубик Д6")]
    Dice,
    #[command(description = "Запустить бот.")]
    Start,
}
