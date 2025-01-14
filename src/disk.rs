use std::fs;
use std::io::Write;
use std::path::PathBuf;

use symlink::symlink_auto;

use crate::{error, warn};

///Save content in a file
pub fn save_file(file_name: &str, content: &[u8], path: &Option<PathBuf>) {
    let path = match path {
        Some(path) => path.join(file_name),
        None => PathBuf::from(file_name),
    };

    if let Some(parent) = path.parent() {
        match fs::create_dir_all(parent) {
            Err(err) => {
                error!("Couldn't create folder {}: {}", parent.display(), err);
            }
            Ok(()) => (),
        }
    }

    let mut file = match fs::File::create(&path) {
        Err(err) => error!("Couldn't create {}: {}", path.display(), err),
        Ok(file) => file,
    };

    if let Err(err) = file.write_all(content) {
        error!("Couldn't write to {}: {}", path.display(), err);
    }
}

///Create a symlink
pub fn symlink(source: &str, destination: &str, path: &Option<PathBuf>) {
    let destination = match path {
        Some(path) => path.join(destination),
        None => PathBuf::from(destination),
    };

    if symlink_auto(source, &destination).is_err() {
        warn!(
            "{} is already present, coulnd't create a symlink to {}",
            destination.display(),
            source,
        );
    }
}
