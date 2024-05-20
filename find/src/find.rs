use std::{env, fs, path::Path, str::FromStr};

use anyhow::Result;
use regex::Regex;

use crate::meta::Params;

pub fn convert_to_regex(patterns: Vec<String>) -> String {
    let regex_patterns: Vec<String> = patterns
        .into_iter()
        .map(|pattern| {
            let mut regex_pattern = regex::escape(&pattern);
            regex_pattern = regex_pattern.replace(r"\*", ".*");
            regex_pattern = regex_pattern.replace(r"\?", ".");
            format!("^{}$", regex_pattern)
        })
        .collect();
    regex_patterns.join("|")
}

pub fn find(params: Params) -> Result<()> {
    // set path
    let path = match params.path {
        Some(d) => Path::new(&d).to_owned(),
        None => env::current_dir().unwrap(),
    };

    // set regex
    let matcher = match params.pattern {
        Some(p) => p,
        None => Regex::from_str("*")?,
    };

    let mut to_ignore = "".to_string();
    // setup gitignore
    if !params.no_ignore {
        let gitignore_path = path.join(".gitignore");
        let gitignore_content = fs::read_to_string(gitignore_path).unwrap_or("".to_string());
        let patterns: Vec<String> = gitignore_content
            .lines()
            .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
            .map(String::from)
            .collect();
        to_ignore = convert_to_regex(patterns);
    }

    let to_ignore_regex: Option<Regex> = if !to_ignore.is_empty() {
        Some(Regex::new(&to_ignore).expect("Invalid regex"))
    } else {
        None
    };

    // iterate
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        if let Some(file_name_str) = file_name.to_str() {
            let file_name_string = String::from(file_name_str);
            let to_ignore_regex = to_ignore_regex.clone();
            if to_ignore_regex.is_some() && to_ignore_regex.unwrap().is_match(&file_name_string) {
                continue;
            }
            if matcher.is_match(&file_name_string) {
                println!("{}", path.to_str().unwrap());
            }
        }
    }
    Ok(())
}
