use std::io::Write;

pub struct LeftPrompt {}

impl LeftPrompt {
    pub fn default() -> Self {
        Self {}
    }

    pub fn draw(&self, dir: String) {
        print!("> {} >>", dir);
        std::io::stdout().flush().unwrap();
    }
}
