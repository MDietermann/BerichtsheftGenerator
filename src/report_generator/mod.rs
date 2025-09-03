use std::fs::File;
use std::io::{self, Write};

pub fn collect_data(content: &str, mut buffer: String) -> String {
    buffer.push_str(content);
    buffer.push_str("\n");
    buffer
}

pub fn write_to_file(buffer: String, file_name: String) -> io::Result<()> {
    let mut file = File::create(&file_name)?;
    file.write_all(buffer.as_bytes())?;
    println!("Successfully wrote to {}", file_name);

    Ok(())
}
