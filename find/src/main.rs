mod find;
mod meta;
use find::convert_to_regex;
use regex::Regex;
use std::{env, process::exit, str::FromStr};

use crate::find::find;
use crate::meta::{get_help, Params, EXIT_FAILURE, EXIT_SUCCESS, VERSION};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut params = Params {
        path: None,
        pattern: None,
        hidden: false,
        no_ignore: false,
        ignore_case: false,
        follow: false,
        max_depth: None,
        exclude: None,
    };

    let mut i = 1;
    let mut max_depth_mode = false;
    let mut exclude_mode = false;

    while i < args.len() {
        // previous was max_depth
        if max_depth_mode {
            let depth = match args[i].parse::<i32>() {
                Ok(d) => {
                    if d < 0 {
                        println!("Max depth should be >0");
                        exit(EXIT_FAILURE)
                    }
                    d
                }
                _ => {
                    println!("Argument for max_depth couldnt be parsed to number");
                    exit(EXIT_FAILURE)
                }
            };
            max_depth_mode = false;
            params.max_depth = Some(depth);
            i += 1;
            continue;
        }

        // previous char was exclude
        if exclude_mode {
            let re = match Regex::from_str(&args[i]) {
                Ok(r) => r,
                _ => {
                    println!("exclude_mode should be a valid regex");
                    exit(EXIT_FAILURE)
                }
            };
            exclude_mode = false;
            params.exclude = Some(re);
            i += 1;
            continue;
        }

        if args[i] == "--version" {
            println!("version: {}\n", VERSION);
            exit(EXIT_SUCCESS)
        } else if args[i] == "--help" {
            println!("{}\n", get_help());
            exit(EXIT_SUCCESS)
        } else if args[i] == "--hidden" {
            params.hidden = true;
        } else if args[i] == "--no-ignore" {
            params.no_ignore = true;
        } else if args[i] == "--ignore-case" {
            params.ignore_case = true;
        } else if args[i] == "--follow" {
            params.follow = true;
        } else if args[i] == "--max-depth" {
            max_depth_mode = true;
        } else if args[i] == "--exclude" {
            exclude_mode = true;
        } else {
            if !(i == args.len() - 1 || i == args.len() - 2) {
                println!("Unkown operation {}", args[i]);
                exit(EXIT_FAILURE)
            }
            if params.pattern.is_none() {
                let patterns: Vec<String> = vec![args[i].to_string()];
                let re = Regex::from_str(&convert_to_regex(patterns));
                params.pattern = match re {
                    Ok(r) => Some(r),
                    _ => {
                        println!("Search pattern: {} is not a regex", args[i]);
                        exit(EXIT_FAILURE)
                    }
                }
            } else {
                params.path = Some(args[i].clone());
            }
        }
        i += 1;
    }
    // uncomment to debug params
    // dbg!(params.clone());
    let _ = find(params);
}
