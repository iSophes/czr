use dialoguer::Confirm;
use tokio::process::Command;

use crate::util::util::{display_error, display_success};

pub async fn push() -> bool {
    loop {
        let should_push = Confirm::new().with_prompt("Want that pushed?").interact();

        if should_push.is_err() {
            display_error("There was an error processing that, please retry.");
            continue;
        }

        if should_push.is_ok_and(|x| !x) {
            display_success("We've committed that for you, happy days!");
            return false;
        }

        break;
    }

    let result = Command::new("git").args(["branch", "-vv"]).output().await;

    println!("{:?}", result.is_ok());

    return true;
}
