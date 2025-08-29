use clap::{
    Parser,
    ValueEnum
};

use chrono::prelude::*;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum TimeEnum {
    Day,
    Week,
    Month
}

const empty: String = String::new();

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
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
    #[arg(short, long)]
    pub repo: String,

    /// Filter Commits by Author
    #[arg(short, long, default_value_t = empty)]
    pub author: String
}
