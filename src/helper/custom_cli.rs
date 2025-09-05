use std::io::*;

pub fn custom_cli_input(message: String) -> String {
    print!("{}: ", message);
    stdout()
        .flush()
        .expect("Failed to flush stdout");

    let mut input: String = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    println!("------------------------------");

    input.trim().to_string()
}