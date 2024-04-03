use std::{
    fs::{self, canonicalize},
    path::PathBuf,
    process::exit,
};

use crate::meta::{Params, EXIT_FAILURE};

pub fn delete_node(path: PathBuf, params: Params) {
    if params.interactive {
        let msg = format!(
            "File: {}\n Enter y to Delete n to cancel",
            path.to_string_lossy()
        );

        let mut rl = rustyline::DefaultEditor::new().unwrap();
        let readline = rl.readline(&msg);
        match readline {
            Ok(line) => println!("Line: {:?}", line),
            Err(_) => println!("No input"),
        }
    }
    if path.is_file() || path.is_symlink() {
        match fs::remove_file(path.clone()) {
            Ok(_) => {
                if params.verbose {
                    println!("Removed file {}", path.to_str().unwrap());
                }
            }
            Err(e) => {
                println!(
                    "Couldnt remove file {}, because {}",
                    path.to_str().unwrap(),
                    e
                );
                exit(EXIT_FAILURE)
            }
        }
    }
    match fs::remove_dir(path.clone()) {
        Ok(_) => {
            if params.verbose {
                println!("Removed directory {}", path.to_str().unwrap());
            }
        }
        Err(e) => {
            println!(
                "Couldnt remove directory {}, because {}",
                path.to_str().unwrap(),
                e
            );
            exit(EXIT_FAILURE)
        }
    }
}

/**
 * Creates a list of paths to delete and add it to a list
 */
pub fn get_canonicalized_paths(root: PathBuf, params: Params) -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = vec![];

    if let Ok(canonicalized_path) = canonicalize(root.clone()) {
        paths.push(canonicalized_path);
    }
    if params.recursive && root.is_dir() {
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let children = get_canonicalized_paths(entry.path(), params.clone());
                paths.extend(children);
            }
        }
    }
    paths
}
