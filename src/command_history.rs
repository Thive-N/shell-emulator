#[allow(dead_code)]
struct Bcommand {
    command: String,
    args: Vec<String>,
}

pub struct CommandHistory {
    previous_commands: Vec<Bcommand>,
}

impl CommandHistory {
    pub fn default() -> Self {
        Self {
            previous_commands: vec![],
        }
    }
    pub fn add(&mut self, command: String, args: Vec<String>) {
        self.previous_commands.push(Bcommand { command, args });
    }
}
