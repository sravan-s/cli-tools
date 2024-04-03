mod meta;
mod rm;

use meta::{get_help, Params, EXIT_SUCCESS, VERSION};
use rm::{delete_node, get_canonicalized_paths};
use std::{env, path::PathBuf, process::exit};

fn work(params: Params) {
    let paths: Vec<PathBuf> = params.paths.iter().map(PathBuf::from).collect();
    let mut nodes = vec![];
    for path in paths {
        let ns = get_canonicalized_paths(path, params.clone());
        for n in ns {
            if nodes.iter().all(|i: &PathBuf| i.to_str() != n.to_str()) {
                nodes.push(n);
            }
        }
    }

    let mut rev_count = nodes.len();
    while rev_count > 0 {
        delete_node(nodes[rev_count - 1].clone(), params.clone());
        rev_count -= 1;
    }
    exit(EXIT_SUCCESS)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut params = Params {
        force: false,
        interactive: false,
        one_file_system: false,
        no_preserve_root: false,
        recursive: false,
        verbose: false,
        paths: Vec::new(),
    };

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--version" {
            println!("version: {}\n", VERSION);
            exit(EXIT_SUCCESS)
        } else if args[i] == "--help" {
            println!("{}\n", get_help());
            exit(EXIT_SUCCESS)
        } else if args[i] == "--force" {
            params.force = true;
        } else if args[i] == "--interactive" {
            params.interactive = true;
        } else if args[i] == "--recursive" {
            params.recursive = true;
        } else if args[i] == "--verbose" {
            params.verbose = true;
        } else {
            params.paths.push(args[i].clone());
        }
        i += 1;
    }
    dbg!(params.clone());
    work(params);
}
