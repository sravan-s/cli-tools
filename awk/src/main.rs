use std::fs;

mod lexer;
fn main() {
    // let sample_1 = fs::read_to_string("./tests/sample_5.awk").expect("Unable to read sample_1.txt");
    let sample_1 = fs::read_to_string("./tests/sample_4.awk").expect("Unable to read sample_1.txt");
    let lexed_1 = lexer::tokenize(sample_1.clone());
    dbg!(sample_1);
    dbg!(lexed_1);
}
