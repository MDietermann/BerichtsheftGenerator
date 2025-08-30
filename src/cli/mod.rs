use clap::{
    Parser,
    ValueEnum,
    Subcommand,
    Args
};

use chrono::prelude::*;
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
    GenerateCommits(GenerateCommitsArgs)
}

const EMPTY: String = String::new();

#[derive(Args, Debug)]
pub struct GenerateCommitsArgs {
    /// Name of the Person to greed
    #[arg(short, value_parser = clap::value_parser!(TimeEnum))]
    pub time: TimeEnum,

    /// Number of times to greed
    #[arg(short, long, default_value_t = Local::now())]
    pub date: DateTime<Local>, 

    /// Owner of the Repository
    #[arg(short, long)]
    pub owner: String,

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
