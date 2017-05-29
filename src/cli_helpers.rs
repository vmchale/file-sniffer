extern crate regex;

use error::check_regex;
use types::FileTree;
use regex::Regex;
use std::path::PathBuf;

pub fn get_dir(path_from_cli: Option<&str>) -> PathBuf {
    if let Some(read) = path_from_cli {
        let path_in = PathBuf::from(read);
        path_in
    }
    else {
        PathBuf::from(".")
    }
}
