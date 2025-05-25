/*
Given a folder full of media files:
1. identify all of the media files
2. create folder with the same name as each file for each file
3. move the media files into those folders
*/

use anyhow::Result;
use std::fs::{create_dir, read_dir, rename};
use std::path::{Path, PathBuf};

const KNOWN_EXTENSIONS: [&str; 8] = ["mkv", "asf", "avi", "mov", "mp4", "mpeg", "mpegts", "wmv"];

/// Check to see if the given file is a recognized media file
fn is_media_file(path: &Path) -> bool {
    fn extract_ext(path: &Path) -> Option<String> {
        Some(path.extension()?.to_ascii_lowercase().to_str()?.to_owned())
    }

    if let Some(ext) = extract_ext(&path) {
        KNOWN_EXTENSIONS.contains(&ext.as_str())
    } else {
        false
    }
}

/// Scan a folder and return all of the known files
/// in the root of that folder.
pub fn list_media_files<P: Into<PathBuf>>(path: P) -> Result<Vec<PathBuf>> {
    let path: PathBuf = path.into();

    let mut media_files: Vec<PathBuf> = Vec::new();

    if path.is_file() & is_media_file(&path) {
        // handle case where user passed a single file
        media_files.push(path);
    } else if path.is_dir() {
        // handle case if user passed folder with files inside

        // scan the input folder and find all of the media files
        for file in read_dir(path)? {
            let file = file?;
            if file.file_type()?.is_file() {
                let fp = file.path();
                if is_media_file(&fp) {
                    media_files.push(fp)
                }
            }
        }
    }

    Ok(media_files)
}

/// Crate a new folder with the same name as the provided file
/// and move the file into that folder.
/// Returns the new path to the file.
pub fn folderize(path: &Path) -> Result<PathBuf> {
    // name will be the path name but without an extension
    let new_folder = path.with_extension("");

    // make the new folder
    create_dir(&new_folder)?;

    let new_path = new_folder.join(path.file_name().expect("should always work"));

    // move the file into the folder we just created
    rename(path, &new_path)?;

    Ok(new_path)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use rand::prelude::*;
    use tempfile::{NamedTempFile, tempdir};

    use super::*;

    // test to make sure that the whole thing works
    // when its put together
    #[test]
    fn test_move() {
        let mut rng = rand::rng();

        // make a tempdir
        let tmp_dir = tempdir().unwrap();

        let mut files = Vec::new();

        // layout a bunch of files
        for _ in 0..10 {
            let suffix = ".".to_owned() + KNOWN_EXTENSIONS.choose(&mut rng).unwrap().to_owned();
            let f = NamedTempFile::with_suffix_in(suffix, &tmp_dir).unwrap();
            files.push(f);
        }

        // run our function
        for file in list_media_files(&tmp_dir.path()).unwrap() {
            folderize(&file).unwrap();
        }

        // compute the expected new locations of the files after they have been moved
        let expected: Vec<PathBuf> = files
            .iter()
            .map(|p| {
                p.path()
                    .with_extension("")
                    .join(p.path().file_name().unwrap())
            })
            .collect();

        // make sure they all exist in the new locations
        for file in expected {
            assert!(file.exists());
        }

        // make sure they no longer exist in the old location
        for file in files {
            assert!(!file.path().exists());
        }

        // check to make sure that all of the files were
        // moved into folders
    }

    // make sure that we only find the files with the approved extensions
    fn test_only_media_files_found() {}
}
