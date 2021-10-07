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

    pub fn run(&mut self) {
        let current_dir = std::env::current_dir().unwrap();
        println!("{}", current_dir.display());

        loop {
            //draw Left prompt
            self.left_prompt.draw(current_dir.display().to_string());

            //get input
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            //split input
            let mut x = input.split(" ");

            //get command
            let command = x.next().unwrap().trim();

            //initialise vector for args
            let mut args: Vec<String> = vec![];

            //unwraps the split object "input" and removs "" values
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

            //adds command to history
            self.command_history.add(command.to_string(), args.clone());

            //initialises a std::process::Command
            let mut command_run = Command::new(command);

            //if there are arguments add them to the command
            if args.len() != 0 {
                command_run.args(args);
            }
            //runs the command
            if let Err(e) = command_run.status() {
                println!("{}", e)
            }
        }
    }
}
