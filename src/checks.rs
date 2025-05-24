use anyhow::Result;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

/// Check the given tree and fill in the given vec with
/// all of the paths found that contain non unicode chars
pub fn check_tree<P: Into<PathBuf>>(path: P) -> Result<Vec<PathBuf>> {
    fn _check_tree(path: &Path, invalid_paths: &mut Vec<PathBuf>) -> Result<()> {
        for file in read_dir(path)? {
            let file = file?;
            let ftype = file.file_type()?;

            // update our ok tracking if the string is not ok
            if file.file_name().to_str().is_none() {
                invalid_paths.push(file.path());
            }

            if ftype.is_dir() {
                _check_tree(&file.path(), invalid_paths)?;
            }
        }

        Ok(())
    }

    let path = path.into();
    let mut invalid_paths = Vec::new();
    _check_tree(&path, &mut invalid_paths)?;

    Ok(invalid_paths)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check() {
        // run on the complete file tree
        let path = PathBuf::from("test_data");
        println!("{:?}", path.canonicalize().unwrap());

        let invalid_paths = check_tree(&path).unwrap();
        assert_eq!(invalid_paths.len(), 1);

        println!("invalid paths found: {:?}", invalid_paths);
    }

    // make sure that when given a folder with only valid
    // paths we dont return a false positive
    #[test]
    fn test_only_valid() {
        let ip = check_tree("test_data/utf").unwrap();
        assert_eq!(ip.len(), 0);
    }
}
