use tokio::process::Command;

use crate::util::util::display_error;

pub async fn check_diffs() -> bool {
    let unstaged_diff = Command::new("git").arg("diff").output().await.unwrap();
    let staged_diff = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .await
        .unwrap();

    let are_changes_staged = unstaged_diff.stdout.is_empty();
    let mut are_staged_empty = staged_diff.stdout.is_empty();

    // nothing staged or to stage

    if are_changes_staged && are_staged_empty {
        display_error("You haven't made any changes. Nothing to commit.");
        return false;
    }

    // stuff to stage

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

            are_staged_empty = false;
        }
    }

    // we have something to commit

    if !are_staged_empty {
        return true;
    }

    display_error("Please push changes to staging before trying to commit, nothing to commit.");
    return false;
}
