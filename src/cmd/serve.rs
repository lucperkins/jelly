use std::{path::PathBuf, process::ExitCode};

pub fn serve(_: PathBuf) -> eyre::Result<ExitCode> {
    Ok(ExitCode::SUCCESS)
}
