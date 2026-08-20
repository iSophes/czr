use std::process::exit;

use clap::Parser;
use dialoguer::{Confirm, Input};
use tokio::process::Command;

use crate::util::util::{display_error, display_info, display_success};

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
    let status = Command::new("git").arg("status").output().await.unwrap();

    if !status.status.success() {
        // not a git repo
        display_error("No git repo has been initialised here.");
        let result = Confirm::new()
            .with_prompt("Would you like to initialise one?")
            .interact()
            .unwrap();

        if !result {
            display_info("Cancelling...");
            exit(0);
        }

        let mut repo_link: String;

        loop {
            repo_link = Input::new()
                .with_prompt("Please input repository link")
                .interact_text()
                .unwrap();

            let is_link_valid = Command::new("git")
                .args(["ls-remote", &repo_link])
                .output()
                .await
                .expect("We had an issue getting the repository link")
                .status
                .success();

            if is_link_valid {
                break;
            }

            display_error("Invalid repository link.");
            continue;
        }

        Command::new("git")
            .arg("init")
            .output()
            .await
            .expect("We had an issue initialising the git repo.");

        Command::new("git")
            .args(["remote", "add", "origin", &repo_link])
            .output()
            .await
            .expect("We had an issue adding the repository.");

        display_success("Successfully initialised git repo.");
    }

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
