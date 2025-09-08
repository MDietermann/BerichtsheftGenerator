mod cli;
mod report_generator;
mod helper;

use std::path::{ PathBuf, Path };
use std::process::{ Command, Stdio };
use std::io::{ BufReader, BufRead };
use clap::{ error::Result, Parser };
use cli::*;
use report_generator::write_to_file;
use clearscreen::clear;
use std::{ env, thread };
use std::sync::{ Arc, Mutex };

use tokio::time::error::Error;

use dotenv::dotenv;

use crate::helper::custom_cli::{custom_cli_input, get_available_repos, git_log_cli};
use crate::report_generator::collect_data;
use crate::helper::user_data::UserData;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = clear();

    let cli = Cli::parse();

    dotenv().ok();

    // let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    match cli.command {
        Commands::Install => {
            install();
        }
        Commands::GenerateCommits => {
            if !Path::new("install.log").exists() {
                install();
            }

            let mut user_data: UserData = UserData::new();

            user_data = user_data
                .get_git_name()
                .get_git_token()
                .get_project_path();

            let all_repos: String = custom_cli_input(format!("Do you want to scan all repositories in {}? [y/N]", user_data.project_path_str));

            if all_repos == "y" || all_repos == "Y" {
                get_all_commits(user_data);
            } else if all_repos == "n" || all_repos == "N" || all_repos.trim().is_empty() {
                custom_cli_input(format!("Which Repository should be scanned"));
            } else {
                println!("Invalid input! Exiting...");
                std::process::exit(0);
            }
         },
    };

    Ok(())
}

fn get_all_commits(user_data: UserData) {

    println!("Scanning all repositories");

    let repos = get_available_repos(user_data.project_path_str.clone());

    // get_date_range(date);

    let mut buffer = String::new();

    for repo_path in repos {
        println!("Scanning repo: {:#?}", repo_path);
        buffer = git_log_cli(buffer, repo_path, user_data.clone());
    }

    _ = write_to_file(buffer, "output.log".to_string());
}



fn install() {
    _ = clear();
    println!("Willkommen beim Berichtsheft Generator!");

    let project_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scripts_path = project_path.join("scripts");

    // Use Stdio::piped to get a handle to the child process's output
    let mut install_command = Command::new("sh")
        .current_dir(scripts_path)
        .arg("init.sh")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn installation script");

    let log_arc = std::sync::Arc::new(Mutex::new(String::new()));

   // Clone the log Arc for the stdout thread
    let log_clone_stdout = Arc::clone(&log_arc);
    let stdout = install_command.stdout.take().expect("Failed to get stdout");

    let stdout_reader = BufReader::new(stdout);
    let stdout_thread = thread::spawn(move || {
        stdout_reader.lines().for_each(|line| {
            if let Ok(l) = line {
                // Lock the mutex to safely modify the log
                let mut log = log_clone_stdout.lock().unwrap();
                println!("{}",  &l);
                *log = collect_data(&l, log.clone());
            }
        });
    });

    // Clone the log Arc for the stderr thread
    let log_clone_stderr = Arc::clone(&log_arc);
    let stderr = install_command.stderr.take().expect("Failed to get stderr");

    let stderr_reader = BufReader::new(stderr);
    let stderr_thread = thread::spawn(move || {
        stderr_reader.lines().for_each(|line| {
            if let Ok(l) = line {
                // Lock the mutex to safely modify the log
                let mut log = log_clone_stderr.lock().unwrap();
                println!("{}",  &l);
                *log = collect_data(&l, log.clone());
            }
        });
    });

    // Wait for the threads to finish processing all output
    stdout_thread.join().unwrap();
    stderr_thread.join().unwrap();
    
    // Now that all output has been collected, write to the file
    let log_final = Arc::try_unwrap(log_arc)
        .unwrap()
        .into_inner()
        .unwrap();

    let _ = write_to_file(log_final, "install.log".to_string());

    // Wait for the command to finish and get the exit status
    let status = install_command.wait().expect("Failed to wait on child process");

    if status.success() {
        println!("Installation completed successfully.");

    } else {
        eprintln!("Installation failed with exit code: {:?}", status.code());
    }
}
