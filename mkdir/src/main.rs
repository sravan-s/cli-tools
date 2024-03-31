use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::{env, process::exit};

const VERSION: &str = "0.0.1";
const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const EXIT_FAIL_PERMISSION: i32 = 2;

struct Options<'a> {
    make_parents: bool,
    mode: u32,
    verbose: bool,
    path: Option<&'a str>,
}

/*
fn canonicalize_path(path_str: &str) -> PathBuf {
    let special_cases = ["./", "..", "~", "/"];
    let pwd = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Couldnt find current path {}", e);
            exit(EXIT_FAILURE)
        }
    };
    if special_cases.iter().any(|x| path_str.starts_with(x)) {
        dbg!(aa)
    } else {
        pwd
    }
}
*/

fn make(options: Options) {
    let path_to_create = match options.path {
        Some(p) => p,
        None => {
            eprintln!("No directory to create, input a path mkdir ./my_dir");
            exit(EXIT_FAILURE)
        }
    };

    if options.make_parents {
        println!("to implement");
    } else {
        let created = match fs::create_dir(path_to_create) {
            Ok(_c) => fs::canonicalize(path_to_create).unwrap(),
            Err(e) => {
                eprintln!("Couldnt craete directory {}", e);
                exit(EXIT_FAILURE)
            }
        };
        let mut permissions = fs::metadata(created.clone())
            .expect("Failed to get metadata")
            .permissions();
        permissions.set_mode(options.mode);

        match fs::set_permissions(created.clone(), permissions) {
            Ok(_) => {
                println!("Directory created successfully.");
                dbg!(created);
                exit(EXIT_SUCCESS)
            }
            Err(e) => {
                eprintln!("Couldnt set permissions to directory {}", e);
                exit(EXIT_FAIL_PERMISSION)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut params = Options {
        make_parents: false,
        mode: 0o755,
        verbose: false,
        path: None,
    };
    /* first arg is usually binary path */
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--version" {
            print!("version: {}\n", VERSION);
            exit(EXIT_SUCCESS)
        }

        if args[i] == "--help" {
            let specs = include_bytes!("../specs.md");
            print!("{}\n", String::from_utf8_lossy(specs));
            exit(EXIT_SUCCESS)
        }

        if args[i] == "-p" {
            params.make_parents = true;
            i += 1;
            continue;
        }

        if args[i] == "-v" {
            params.verbose = true;
            i += 1;
            continue;
        }

        if args[i] == "-m" {
            let mode: u32 = match args[i + 1].parse() {
                Ok(num) => num,
                Err(_) => {
                    eprintln!("Failed to parse permission string: '{}'", args[i + 1]);
                    exit(EXIT_FAILURE);
                }
            };
            params.mode = mode;
            i = i + 2;
            continue;
        }

        match params.path {
            Some(_) => {
                eprintln!("Unknown paramter {}", args[i]);
                exit(EXIT_FAILURE);
            }
            None => {
                params.path = Some(&args[i]);
                i += 1;
            }
        }
    }

    make(params);
}
