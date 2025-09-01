use std::fs::File;
use std::io::{self, Write};

pub fn collect_data(content: &str, mut buffer: String) -> String {
    buffer.push_str(content);

    buffer
}

pub fn write_to_file(content: String) -> io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(content.as_bytes())?;
    println!("Successfully wrote to output.txt");

    Ok(())
}
