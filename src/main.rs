use std::{process::exit, time::Duration};

use clap::Parser;
use tokio::{process::Command, time};

use crate::util::util::{clear_console, display_error, display_info, display_success};

mod util;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of commit to commit.
    #[arg(short)]
    commit_type: Option<String>,
}

#[tokio::main]
async fn main() {
    //clear_console();
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

    let unstaged_diff = Command::new("git").arg("diff").output().await.unwrap();
    let staged_diff = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .await
        .unwrap();
    let mut are_changes_staged = unstaged_diff.stdout.is_empty();
    let mut are_staged_empty = staged_diff.stdout.is_empty();

    if are_changes_staged && are_staged_empty {
        display_error("You haven't made any changes. Nothing to commit.");
        exit(0);
    }

    if !are_changes_staged {
        let result = dialoguer::Confirm::new()
            .with_prompt("You haven't staged all your commits. Would you like to?")
            .interact()
            .unwrap();

        if result {
            Command::new("git")
                .arg("add")
                .arg(".")
                .output()
                .await
                .expect("Failed to stage all commits.");

            are_changes_staged = true;
            are_staged_empty = false;
        }
    }

    if are_changes_staged && !are_staged_empty {
        // begin commit process!!!

        println!("we can begin commit messaging!!!!!")
    }
}
