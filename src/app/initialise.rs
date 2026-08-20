use crate::util::util::{display_error, display_info, display_success};
use dialoguer::{Confirm, Input};
use owo_colors::OwoColorize;
use tokio::process::Command;

pub fn request_init() -> bool {
    let result = Confirm::new()
        .with_prompt("Would you like to initialise one?")
        .interact()
        .unwrap();

    if !result {
        display_info("Cancelling...");
        return false;
    }

    return true;
}

pub async fn request_link() -> String {
    loop {
        let repo_link: String = Input::new()
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
            return repo_link;
        }

        display_error("Invalid repository link.");
        continue;
    }
}

async fn is_head_initialised() -> bool {
    Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .output()
        .await
        .is_ok()
}

pub async fn initialise_repo() -> bool {
    // not a git repo
    display_error("No git repo has been initialised here.");

    if !request_init() {
        return false;
    }

    let repo_link: String = request_link().await;

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

    if !is_head_initialised().await {
        let branch: String = Input::new()
            .with_prompt(format!(
                "{text} {example}",
                text = "No HEAD branch detected, please insert one".white(),
                example = "(Example: master)".default_color()
            ))
            .interact_text()
            .unwrap();

        Command::new("git")
            .args(["branch", "-M", &branch])
            .output()
            .await
            .expect("We had an issue adding that branch.");

        Command::new("git")
            .args(["push", "-u", "origin", &branch])
            .output()
            .await
            .expect("We had an issue pushing to that branch.");
    }

    display_success("Successfully initialised git repo.");
    return true;
}
