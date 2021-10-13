use crate::command_history::CommandHistory;
use crate::left_prompt::LeftPrompt;
use std::collections::HashMap;

use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Shell {
    command_history: CommandHistory,
    left_prompt: LeftPrompt,
    alias: HashMap<String, (String, Vec<String>)>,
    path: PathBuf,
}
impl Shell {
    pub fn default() -> Self {
        Shell {
            command_history: CommandHistory::default(),
            left_prompt: LeftPrompt::default(),
            alias: HashMap::new(),
            path: std::env::current_dir().unwrap(),
        }
    }

    pub async fn run(&mut self) {
        self.alias
            .insert("la".to_string(), ("ls".to_string(), vec!["-a".to_string()]));
        self.alias.insert(
            "lla".to_string(),
            ("ls".to_string(), vec!["-la".to_string()]),
        );

        loop {
            self.left_prompt.draw(self.path.clone()).await;
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut x = input.split(" ");

            let mut command = x.next().unwrap().trim();

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

            if command == "" {
                continue;
            }

            self.command_history.add(command.to_string(), args.clone());
            if command == "cd" {
                if Path::new(&args[0]).exists() {
                    self.path = PathBuf::from(&args[0])
                } else {
                    println!("path doesnt exist")
                }
                continue;
            }

            if self.alias.contains_key(command) {
                let temp = self.alias.get_key_value(command).unwrap();
                command = &temp.1 .0;
                args = temp.1 .1.clone();
            }
            let mut command_run = Command::new(command);

            if args.len() != 0 {
                command_run.args(args);
            }

            command_run.current_dir(&self.path);

            if let Err(e) = command_run.status() {
                println!("{}", e)
            }
        }
    }
}
