#[derive(Debug, Clone)]
pub struct Params {
    pub force: bool,
    pub interactive: bool,
    pub one_file_system: bool,
    pub no_preserve_root: bool,
    pub recursive: bool,
    pub verbose: bool,
    pub paths: Vec<String>,
}

pub const VERSION: &str = "0.0.1";
pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

pub fn get_help() -> String {
    let specs_u8 = include_bytes!("../README.md");
    let specs: String = String::from_utf8_lossy(specs_u8).to_string();
    specs
}
