pub mod command_history;
pub mod left_prompt;
use tokio;
mod shell;
use shell::Shell;

#[tokio::main]
async fn main() {
    let mut shell = Shell::default();
    shell.run().await;
}
