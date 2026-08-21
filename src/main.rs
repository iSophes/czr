use std::process::exit;

use clap::{Parser, Subcommand};
use tokio::process::Command;

use crate::{
    app::{
        app_config::{self, Settings},
        construct::construct_message,
        diff, initialise,
        push::push,
    },
    util::util::display_success,
};

mod app;
mod util;

/// CZR is a program to help you write git commits.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Configure the tool. Currently only contains two configurable values.
    Config {},
}

async fn run_program(new_settings: Settings) {
    let status = Command::new("git").arg("status").output().await.unwrap();

    if !status.status.success() {
        let success = initialise::initialise_repo().await;
        if !success {
            exit(0);
        }
    }

    if !diff::check_diffs().await {
        exit(0)
    }

    if !construct_message(new_settings).await {
        exit(0)
    }

    display_success("Commit successful.");

    if !push().await {
        exit(0)
    }
}

#[tokio::main]
async fn main() {
    let mut new_settings = app_config::Settings::new();
    let _ = new_settings.setup();

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Config {}) => {
            let result = new_settings.call_settings();
            if result.is_ok() {
                display_success("Configuration successful.");
                exit(0);
            }
        }
        _ => run_program(new_settings).await,
    }
}
