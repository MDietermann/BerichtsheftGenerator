use clap::{
    Parser,
    ValueEnum,
    Subcommand,
    Args
};

use std::string::String;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimeEnum {
    Day,
    Week,
    Month
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Fresh Install (Has to be run at least once!)
    Install,

    /// Generate Document with Git Commits
    GenerateCommits,
}

const EMPTY: String = String::new();

#[derive(Args, Debug)]
pub struct GenerateCommitsArgs {
    /// Date range to check (day, week, month)
    #[arg(short, value_parser = clap::value_parser!(TimeEnum))]
    pub date_range: TimeEnum,

    /// Date that lies in the given date range Example: 2025-08-13
    #[arg(short, long)]
    pub date: String, 

    #[arg(short, long)]
    /// Path to the Projects folder for the repositories to check
    pub projects_path: String,

    /// Repository to check
    #[arg(short, long, default_value_t = String::from("all"))]
    pub repo: String,

    /// Filter Commits by Author
    #[arg(short, long, default_value_t = EMPTY)]
    pub author: String
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
