pub mod command_history;
pub mod left_prompt;

mod shell;
use shell::Shell;

fn main() {
    let mut shell = Shell::default();
    shell.run();
}
