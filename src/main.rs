use std::fs;

fn main() {
    let file_path = "lexer_input_test.zig";
    let separate_chars = [' ', '\n', '\r', '\t'];
    println!("Reading {file_path}");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("Text with:\n{contents}");
    let tokens : Vec<&str> = contents.split(&separate_chars[..]).collect();
    println!("Unfiltred:\n{:?}",tokens);
    let filtred_tokens : Vec<&str> = tokens.into_iter().filter(|&x| !x.is_empty()).collect();
    println!("Filtered Tokens:\n{:?}",filtred_tokens);
}
