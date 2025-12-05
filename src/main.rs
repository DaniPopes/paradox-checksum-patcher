use color_eyre::{Result, eyre};
use std::{env, io, path::Path};
use tracing_subscriber::EnvFilter;

#[macro_use]
extern crate tracing;

mod patch;

const SUPPORTED_EXES: &[&str] = &["eu4.exe", "eu5.exe", "hoi4.exe"];

fn main() -> Result<()> {
    color_eyre::install()?;

    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(tracing::Level::INFO.into())
                .from_env_lossy(),
        )
        .init();

    if let Err(e) = run() {
        error!("{e}");
        debug!("{e:?}");
        info!("patch wasn't installed, no files have been changed");
    }

    info!("press enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();

    if !args.is_empty() {
        let mut patched_any = false;

        for arg in &args {
            let path = Path::new(arg);
            if path.exists() {
                if path.is_file() {
                    patch_file(path)?;
                    patched_any = true;
                } else if path.is_dir() {
                    patch_directory(path)?;
                    patched_any = true;
                }
            } else {
                info!("path not found, skipping: {}", path.display());
            }
        }

        if patched_any {
            return Ok(());
        }

        debug!("no valid paths provided, searching current directory");
    }

    patch_directory(Path::new("."))
}

fn patch_file(path: &Path) -> Result<()> {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if !SUPPORTED_EXES.contains(&filename) {
        eyre::bail!(
            "unsupported executable: {}. Supported: {}",
            filename,
            SUPPORTED_EXES.join(", ")
        );
    }

    patch_single_file(path)
}

fn patch_directory(dir: &Path) -> Result<()> {
    let files_in_dir = std::fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_file()).unwrap_or(false))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();

    let files_to_patch: Vec<_> =
        files_in_dir.iter().map(String::as_str).filter(|f| SUPPORTED_EXES.contains(f)).collect();

    if files_to_patch.is_empty() {
        eyre::bail!("cannot locate supported game executable in directory: {}", dir.display());
    }

    for file in &files_to_patch {
        info!("found {} in directory {}", file, dir.display());
    }

    for file in files_to_patch {
        patch_single_file(&dir.join(file))?;
    }

    Ok(())
}

fn patch_single_file(path: &Path) -> Result<()> {
    info!("patching {}", path.display());
    patch::apply_patch(path)?;
    info!(
        "patch successfully installed, original executable has been backed up in {}.backup",
        path.display()
    );
    Ok(())
}
