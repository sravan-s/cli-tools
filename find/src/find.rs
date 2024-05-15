use std::{env, fs, path::Path, str::FromStr};

use anyhow::Result;
use regex::bytes::Regex;

use crate::meta::Params;

pub fn find(params: Params) -> Result<()> {
    let path = match params.path {
        Some(d) => Path::new(&d).to_owned(),
        None => env::current_dir().unwrap(),
    };

    let matcher = match params.pattern {
        Some(p) => p,
        None => Regex::from_str("*")?,
    };

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if let Some(file_name_str) = file_name.to_str() {
            let file_name_string = String::from(file_name_str);
            if matcher.captures(file_name_string.as_bytes()).is_some() {
                println!("{}", path.to_str().unwrap());
            }
        }
        // println!("{}", path.to_str().unwrap());
    }
    Ok(())
}
