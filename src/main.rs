use anyhow::Result;
use std::io;
use tracing::{error, info};

mod patch;

const SUPPORTED_EXES: &[&str] = &["eu4.exe", "eu5.exe", "hoi4.exe"];

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    if let Err(e) = run() {
        error!("{}", e);
        info!("patch wasn't installed, no files have been changed");
    }

    info!("press enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}

fn run() -> Result<()> {
    let files_in_dir = std::fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();

    let files_to_patch: Vec<_> = files_in_dir
        .iter()
        .filter(|f| SUPPORTED_EXES.contains(&f.as_str()))
        .collect();

    if files_to_patch.is_empty() {
        anyhow::bail!("cannot locate supported game executable in current directory");
    }

    for file in &files_to_patch {
        info!("found {} in current directory", file);
    }

    for file in files_to_patch {
        info!("patching {}", file);
        patch::apply_patch(file)?;
        info!(
            "patch successfully installed, original executable has been backed up in {}.backup",
            file
        );
    }

    Ok(())
}
