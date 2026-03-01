use anyhow::Context;
use explora::ui::Explorer;

fn main() -> anyhow::Result<()> {
    
    let cwd = std::env::current_dir()
        .with_context(|| "Current working directory is invalid or has insufficient permisions.")?
        .to_path_buf();
    
    ratatui::run(|terminal| -> anyhow::Result<()> {
        let mut explorer = Explorer::new(cwd);
        
        explorer.run(terminal)
            .with_context(|| "Explorer loop failed.")?;
        Ok(())
    })
    .with_context(|| "Main application logic failed.")?;
    
    Ok(())
}
