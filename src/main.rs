mod cli;
mod report_generator;

use clap::Parser;
use clearscreen::clear;
use cli::Cli;
use cli::Commands;
use report_generator::write_to_file;
use std::process::Command;
use tokio::time::error::Error;
use walkdir::WalkDir;

use dotenv::dotenv;
use std::env;

use crate::report_generator::collect_data;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let cli = Cli::parse();

    dotenv().ok();

    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");

    match cli.command {
        Commands::Install => {
            install();
        }
        Commands::GenerateCommits(generate_commits_args) => {
            let owner = generate_commits_args.owner;
            let repo = generate_commits_args.repo;
            let commit_author = generate_commits_args.author;

            println!("Fetching commits for {}/{}", owner, repo);

            get_commits(repo);
        }
    };

    Ok(())
}

fn get_commits(repo: String) {
    let mut repos = Vec::new();
    for entry in WalkDir::new("/home/marvind/git-clones")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            // Check if the directory contains a .git subdirectory
            let git_path = entry.path().join(".git");
            if git_path.exists() && git_path.is_dir() {
                repos.push(entry.path().to_path_buf());
            }
        }
    }

    let mut buffer = String::new();

    if repo == "all" {
        for repo_path in repos {
            let mut git_log_command = Command::new("git");
            git_log_command.current_dir(repo_path);

            git_log_command.arg("log").arg("--all");

            match git_log_command.output() {
                Ok(output) => {
                    if output.status.success() {
                        // Print the standard output as a string
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        buffer = collect_data(&stdout, buffer);
                    } else {
                        // Print the standard error
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("Command failed with error:\n{}", stderr);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                }
            }
        }

        _ = write_to_file(buffer);
    }
}

fn install() {
    _ = clear();
    println!("Willkommen beim Berichtsheft Generator!")
}
