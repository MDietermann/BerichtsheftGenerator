mod cli;

use clap::Parser;
use cli::Args;
use octocrab::{
    Octocrab,
    models::Repository,
    Page
};

use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let args = Args::parse();

    dotenv().ok();

    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
    let octo = Octocrab::builder().personal_token(token).build()?;

    let owner = args.owner;
    let repo = args.repo;
    let commit_author = args.author;

    println!("Fetching commits for {}/{}", owner, repo);

    // Get the page of commits. We can now use `octo.commits()` to target a specific repo.
    let commits_page = octo
        .repos(owner, repo)
        .list_commits()
        .per_page(30)
        .send()
        .await?;

    for commit in commits_page.items {
        if let Some(author) = commit.commit.author {
            // Check if the date is Some(date) before calling the method.
            if let Some(date) = author.date {
                if commit_author == author.name || commit_author == String::new() {
                    println!(
                        " - Commit by {}: {}",
                        author.name,
                        date.to_rfc3339()
                    );
                }

            } else {
                println!(" - Commit by {}: Date not available", author.name);
            }
            if commit_author == author.name || commit_author == String::new() {
                println!("   Message: {}", commit.commit.message.lines().next().unwrap_or("No message"));
            }
        }
    }

    Ok(())
}
