mod checks;
mod hello;
mod media_mover;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

/// Define the different actions that
/// can be done with this tool
#[derive(Debug, Subcommand)]
enum Action {
    /// Say hello
    SayHello,
    /// Check that all files and folders at the given
    /// tree are using UTF8
    CheckUTF8 { path: PathBuf },
    /// Move all media files in the target folder
    /// into new folders with the same name as the
    /// media file.
    ///
    /// If a media file instead of a folder is passed as
    /// the argument then it moves only that one file.
    MoveToFolders { path: PathBuf },
}

/// Simple tool for doing a handful of plex related tasks.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.action {
        Action::SayHello => hello::say_hello(),
        Action::CheckUTF8 { path } => {
            println!("Checking path: {path:#?}");
            let invalid = checks::check_tree(path)?;
            if invalid.is_empty() {
                println!("All files ok!");
            } else {
                println!("Found {} invalid files!", invalid.len());
                for p in invalid {
                    println!("\t{p:#?}");
                }
            }
        }
        Action::MoveToFolders { path } => {
            println!("Scanning: {:#?} to locate media files...", &path);
            let media_files = media_mover::list_media_files(&path)?;
            println!("Found {} files!", media_files.len());

            for file in media_files {
                println!("Moving: {:?}", &file);
                media_mover::folderize(&file)?;
            }
        }
    };

    Ok(())
}
