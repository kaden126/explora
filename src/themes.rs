use anyhow::{Result, Context};
use ratatui::style::{Style};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Theme {
    pub app: Style,
    pub audio: Style,
    pub image: Style,
    pub archive: Style,
    pub doc: Style,
    pub text: Style,
    pub video: Style,
    pub book: Style,
    pub font: Style,
    pub directory: Style,
    pub symlink: Style,
    pub other: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            app: Style::new().red(),
            audio: Style::new().yellow(),
            image: Style::new().green(),
            archive: Style::new().cyan(),
            directory: Style::new().blue(),
            video: Style::new().magenta(),
            other: Style::new().gray(),
            
            font: Style::new().red().underlined(),
            book: Style::new().yellow().underlined(),
            doc: Style::new().green().underlined(),
            symlink: Style::new().cyan().underlined(),
            text: Style::new().gray().underlined(),
        }
    }
}

impl Theme {
    pub fn new() -> Result<Self> {
        confy::load("explora", "theme")
            .with_context(|| "Failed to load theme files.")
    }
}