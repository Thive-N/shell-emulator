use std::io::Write;
use std::path::PathBuf;
use termion::color;
use users::get_current_username;

pub struct LeftPrompt {}

impl LeftPrompt {
    pub fn default() -> Self {
        Self {}
    }
    pub async fn draw(&self, dir: PathBuf) {
        let mut dd = dir.display().to_string();

        match get_current_username() {
            Some(uname) => {
                dd = dd.replacen(
                    &("/home/".to_string() + &uname.into_string().unwrap()),
                    "~",
                    1,
                )
            }
            None => dd = dd.replacen("/home/", "~/", 1),
        }

        print!(
            "{}{} {}❯{}❯{}❯ {}",
            color::Fg(color::Blue),
            dd,
            color::Fg(color::Red),
            color::Fg(color::Yellow),
            color::Fg(color::Green),
            color::Fg(color::Reset)
        );
        std::io::stdout().flush().unwrap();
    }
}
