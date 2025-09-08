use std::{io::*, path::PathBuf, process::Command};

use walkdir::WalkDir;

use crate::{helper::user_data::UserData, report_generator::collect_data};

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


pub fn get_available_repos(path: String) -> Vec<PathBuf> {
    let mut repos = Vec::new();
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            // Check if the directory contains a .git subdirectory
            let git_path = entry.path().join(".git");
            if git_path.exists() && git_path.is_dir() {
                println!("Found: {:#?}", entry);
                repos.push(entry.path().to_path_buf());
            }
        }
    }

    repos
}

pub fn git_log_cli(mut buffer: String, repo_path: PathBuf, user_data: UserData) -> String {
    let command_string = format!("git log --author=\"{}\" --pretty=format:\"Author: %an; Commit-Date: %ad; Message: %s; Repository: $(basename $(git rev-parse --show-toplevel))\" --date=format:'%d.%m.%y, %H:%M'", user_data.git_name);

    let output = Command::new("sh")
        .arg("-c")
        .arg(command_string)
        .current_dir(repo_path)
        .output()
        .expect("Failed to execute command.");
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        buffer = collect_data(&stdout, buffer);
    } else {
        // Print the standard error
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error:\n{}", stderr);
    }

    buffer
}

// pub fn get_date_range(target_date: DateTime<Local>) {
//     let year = target_date.year();
//     print!("{}", year);
// }