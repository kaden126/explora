use std::{
    ffi::OsStr, path::{Path, PathBuf}, time::Duration
};
use infer::MatcherType as FileType;
use anyhow::Context;
use crossterm::event::{Event, KeyCode, poll, read};
use ratatui::{
    DefaultTerminal, Frame, style::{Style, Stylize}, text::{Span, Line}, widgets::{Block, BorderType, List, ListState}
};

fn match_file_type<P: AsRef<Path>>(file: &P) -> Span<'_> {
    let string = Span::raw(file.as_ref().file_name().unwrap_or_else(|| OsStr::new("<no filename>")).to_string_lossy());
    
    if let Ok(Some(kind)) = infer::get_from_path(file) {        
        return match kind.matcher_type() {
            FileType::App => string.light_red(),
            FileType::Audio => string.light_yellow(),
            FileType::Image => string.light_green(),
            FileType::Archive => string.light_cyan(),
            FileType::Doc => string.light_blue(),
            FileType::Text => string.light_magenta(),
            FileType::Video => string.red().underlined(),
            FileType::Book => string.yellow().underlined(),
            FileType::Font => string.green().underlined(),
            FileType::Custom => string.magenta().underlined()
        };
    }
    
   string.magenta().underlined()
}

pub struct Explorer {
    current: PathBuf,
    children: Vec<PathBuf>,
    exit: bool,
    needs_clear: bool
}

impl Explorer {
    
    pub fn new(current: PathBuf) -> Self {
        Self {
            current,
            children: vec![],
            exit: false,
            needs_clear: false,
        }
    }
    
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let mut state = ListState::default().with_selected(Some(0));
        self.set_children()
            .with_context(|| "Failed to draw initial directory.")?;
        while !self.exit {
            self.update(&mut state)
                .with_context(|| "Failed to update explorer state.")?;

            if self.needs_clear {
                terminal.clear()
                    .with_context(|| "Failed to refresh screen after returning from external process.")?;
                self.needs_clear = false;
            }
            
            terminal
                .draw(|frame| {
                    self.draw(frame, &mut state);
                })
                .with_context(|| "Failed to draw to terminal.")?;
        }
        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame, state: &mut ListState) {
        let children = self.children.iter()
            .map(|child| {
                let string = Span::raw(child.file_name().unwrap_or_else(|| OsStr::new("<no filename>")).to_string_lossy());
                
                if child.is_dir() {
                    string.blue().underlined()
                }
                else if child.is_symlink() {
                    string.cyan().underlined()
                }
                else if child.is_file() {
                    match_file_type(child)
                }
                else {
                    string.magenta().underlined()
                }
            });
        
        let list = List::new(children)
            .highlight_style(Style::new().reversed())
            .block(
                Block::bordered().border_type(BorderType::Rounded)
                    .title_bottom(
                        Line::from(vec![
                            Span::from(" "),
                            Span::from("<UP/DOWN>").cyan(),
                            Span::from(" = navigation "),
                            Span::from("<LEFT>").cyan(),
                            Span::from(" = parent "),
                            Span::from("<RIGHT>").cyan(),
                            Span::from(" = child (if dir) "),
                            Span::from("<ENTER>").cyan(),
                            Span::from(" = edit "),
                            Span::from("<ESC>").cyan(),
                            Span::from(" = quit ")
                        ])
                    )
                    .title_top(
                        Line::from(vec![
                            Span::from(" "),
                            Span::from(self.current.to_string_lossy()).blue().underlined(),
                            Span::from(" "),
                            Span::from("bin").light_red(),
                            Span::from(" "),
                            Span::from("audio").light_yellow(),
                            Span::from(" "),
                            Span::from("img").light_green(),
                            Span::from(" "),
                            Span::from("arch").light_cyan(),
                            Span::from(" "),
                            Span::from("doc").light_blue(),
                            Span::from(" "),
                            Span::from("txt").light_magenta(),
                            Span::from(" "),
                            Span::from("vid").red().underlined(),
                            Span::from(" "),
                            Span::from("book").yellow().underlined(),
                            Span::from(" "),
                            Span::from("font").green().underlined(),
                            Span::from(" "),
                            Span::from("link").cyan().underlined(),
                            Span::from(" "),
                            Span::from("dir").blue().underlined(),
                            Span::from(" "),
                            Span::from("other").magenta().underlined(),
                            Span::from(" ")
                        ])
                    )
            );

        frame.render_stateful_widget(&list, frame.area(), state);
    }

    pub fn update(&mut self, state: &mut ListState) -> anyhow::Result<()> {
        let available = poll(Duration::ZERO).with_context(|| "Failed to poll terminal events.")?;

        if !available {
            return Ok(());
        }

        let event = read().with_context(|| "Failed to read terminal events.")?;

        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Down => state.select_next(),
                KeyCode::Up => state.select_previous(),
                KeyCode::Left => self.parent(state)
                    .with_context(|| "Failed to open parent directory.")?,
                KeyCode::Right => self.child(state)
                    .with_context(|| "Failed to open child directory.")?,
                KeyCode::Enter => 
                {
                    if let Some(selected) = self.get_selected(state)
                    {
                        crate::exit::spawn_editor(selected.to_string_lossy().to_string().as_str())
                            .with_context(|| "Failed to open editor.")?;
                        self.needs_clear = true;
                    }
                },
                KeyCode::Esc => self.exit = true,
                _ => (),
            }
        }

        Ok(())
    }

    fn parent(&mut self, state: &mut ListState) -> anyhow::Result<()> {
        self.current = self
            .current
            .parent()
            .unwrap_or_else(|| &self.current)
            .to_path_buf();

        if self.children.is_empty() {
            state.select(None);
        }
        else {
            state.select_first();
        }
        self.set_children()
            .with_context(|| "Failed to set children for opened directory")?;
        Ok(())
    }
    
    fn get_selected(&mut self, state: &mut ListState) -> Option<PathBuf> {
        if let Some(selected_num) = state.selected() {
            let currently_selected = &self.children[selected_num];
            return Some(currently_selected.to_path_buf());
        }
        None
    }

    fn child(&mut self, state: &mut ListState) -> anyhow::Result<()> {
        if let Some(selected_num) = state.selected() {
            let currently_selected = &self.children[selected_num];
            
            if currently_selected.is_dir() {
                self.current = currently_selected.to_path_buf();
            }
        }
        self.set_children()
            .with_context(|| "Failed to set children for opened directory")?;
        Ok(())
    }

    fn set_children(&mut self) -> anyhow::Result<()> {
        self.children = std::fs::read_dir(&self.current)
            .with_context(|| "Failed to iterate over current directory.")?
            .filter_map(|child| child.ok())
            .map(|child| child.path())
            .collect();
        
        Ok(())
    }
}
