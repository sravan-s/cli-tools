use regex::bytes::Regex;

pub const VERSION: &str = "0.0.1";

#[derive(Debug, Clone)]
pub struct Params {
    pub pattern: Option<Regex>,
    pub path: Option<String>,
    pub hidden: bool,      //                     Search hidden files and directories
    pub no_ignore: bool,   //                  Do not respect .(git|fd)ignore files
    pub ignore_case: bool, //                Case-insensitive search (default: smart case)
    pub follow: bool,      //                     Follow symbolic links
    pub max_depth: Option<i32>, //                   Set maximum search depth (default: none),
    pub exclude: Option<Regex>, //                  Exclude entries that match the given glob pattern
}

pub fn get_help() -> String {
    let specs_u8 = include_bytes!("../specs.md");
    let specs: String = String::from_utf8_lossy(specs_u8).to_string();
    specs
}

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;
