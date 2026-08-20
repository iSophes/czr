use std::{process::exit, time::Duration};

use clap::Parser;
use tokio::{process::Command, time};

use crate::util::util::{clear_console, display_error, display_info, display_success};

mod util;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of commit to commit.
    #[arg(short)]
    commit_type: Option<String>,
}

#[tokio::main]
async fn main() {
    clear_console();
    let status = Command::new("git").arg("status").output().await.unwrap();

    if !status.status.success() {
        // not a git repo
        display_error("No git repo has been initialised here.");
        let result = dialoguer::Confirm::new()
            .with_prompt("Would you like to initialise one?")
            .interact()
            .unwrap();

        if !result {
            display_info("Cancelling...");
            exit(0);
        }

        Command::new("git")
            .arg("init")
            .output()
            .await
            .expect("We had an issue initialising the git repo.");

        display_success("Successfully initialised git repo.");
        time::sleep(Duration::from_secs(2)).await;
        clear_console();
    }

    drop(status);
}
