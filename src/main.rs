use std::process::exit;

use clap::Parser;
use tokio::process::Command;

use crate::{
    app::{app_config, construct::construct_message, diff, initialise, push::push},
    util::util::display_success,
};

mod app;
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
    let mut new_settings = app_config::Settings::new();
    let _ = new_settings.setup();

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
