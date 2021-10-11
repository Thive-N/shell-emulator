use crate::command_history::CommandHistory;
use crate::left_prompt::LeftPrompt;

use std::io::stdin;
use std::process::Command;

pub struct Shell {
    command_history: CommandHistory,
    left_prompt: LeftPrompt,
}
impl Shell {
    pub fn default() -> Self {
        Shell {
            command_history: CommandHistory::default(),
            left_prompt: LeftPrompt::default(),
        }
    }

    pub async fn run(&mut self) {
        let current_dir = std::env::current_dir().unwrap();

        loop {
            self.left_prompt.draw(current_dir.clone()).await;

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut x = input.split(" ");

            let command = x.next().unwrap().trim();

            let mut args: Vec<String> = vec![];

            loop {
                match x.next() {
                    Some(x) => {
                        if x != "" {
                            args.push(x.trim().to_string())
                        }
                    }
                    None => break,
                }
            }

            self.command_history.add(command.to_string(), args.clone());

            let mut command_run = Command::new(command);

            if args.len() != 0 {
                command_run.args(args);
            }

            if let Err(e) = command_run.status() {
                println!("{}", e)
            }
        }
    }
}
