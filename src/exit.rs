use std::process::Command;

use anyhow::Context;

pub fn spawn_editor(file: &str) -> anyhow::Result<()> {
    let editor = std::env::var("EDITOR")
        .unwrap_or_else(|_| "vim".to_string());

    Command::new(&editor)
        .arg(file)
        .spawn()   
        .with_context(|| format!("Failed to open editor `{editor}`."))?
        .wait_with_output()
        .with_context(|| format!("Failed to wait for editor `{editor}` to exit."))?;
    Ok(())
}