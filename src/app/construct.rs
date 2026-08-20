use std::vec;

use dialoguer::FuzzySelect;
use owo_colors::OwoColorize;
use tokio::process::Command;

use crate::{app::app_config, util::util::display_error};

pub async fn get_commit_type(settings: app_config::Settings) -> String {
    let mut item_strings = vec![] as vec::Vec<String>;

    for x in settings.commit_types.into_iter() {
        let converted_commit_type = ToString::to_string(&x);
        let description = settings
            .commit_descriptions
            .get(&converted_commit_type)
            .expect(&format!(
                "Description for {val} doesn't exist.",
                val = converted_commit_type
            ))
            .to_owned()
            .to_string();

        if settings.enable_emojis {
            let mut emoji_string = settings
                .commit_emojis
                .get(&converted_commit_type)
                .expect(&format!(
                    "Emoji {emoji} doesn't exist.",
                    emoji = converted_commit_type
                ))
                .to_owned()
                .to_string();

            emoji_string.push_str(&format!(
                " {commit_type}: {desc}",
                commit_type = converted_commit_type,
                desc = description
            ));

            item_strings.push(emoji_string);
            continue;
        }

        item_strings.push(format!(
            "{converted_commit_type}: {description}",
            converted_commit_type = converted_commit_type,
            description = description
        ));
    }

    let selected_type = FuzzySelect::new()
        .items(&item_strings)
        .with_prompt(&format!(
            "{} {} {}",
            "?".bright_green().bold(),
            "Select the type of change you're committing:"
                .white()
                .bold(),
            "(Use arrow keys or type to search)".white()
        ))
        .interact()
        .unwrap();

    let commit_type = if settings.enable_commit_names {
        item_strings[selected_type]
            .split(":")
            .next()
            .expect("Shouldn't be possible to get rid of the colon.")
            .to_owned()
            .split(" ")
            .nth(1)
            .expect("Shouldn't be possible to get rid of the space.")
            .to_owned()
    } else {
        "".to_string()
    };

    let emoji_code = if settings.enable_commit_names {
        &format!("{e} ", e = settings.commit_codes.get(&commit_type).unwrap())
    } else {
        ""
    };

    return format!("{code}{type}", code=emoji_code, type=commit_type);
}

async fn get_short_description() -> String {
    loop {
        let short: String = dialoguer::Input::new()
            .with_prompt(&format!(
                "{question} {text} {chars}",
                question = "?".bright_green().bold(),
                text = "Write a short description".bold().white(),
                chars = "(Max 75 characters.)".default_color()
            ))
            .interact_text()
            .unwrap();

        if short.len() > 75 {
            display_error(&format!(
                "Short description too long! You're at {chars} characters with that.",
                chars = short.len()
            ));
            continue;
        }

        return short;
    }
}

async fn get_long_description() -> String {
    let long: String = dialoguer::Input::new()
        .with_prompt(&format!(
            "{question} {text}",
            question = "?".bright_green().bold(),
            text = "Write a longer description".bold().white(),
        ))
        .allow_empty(true)
        .interact_text()
        .unwrap();

    return long;
}

async fn closes() -> String {
    let closes: String = dialoguer::Input::new()
        .with_prompt(&format!(
            "{question} {text} {info}",
            question = "?".bright_green().bold(),
            text = "List any closed issues".bold().white(),
            info = "(#1, #2, ...)".default_color()
        ))
        .allow_empty(true)
        .interact_text()
        .unwrap();

    return closes;
}

pub async fn construct_message(settings: app_config::Settings) -> bool {
    let commit_type = get_commit_type(settings).await;
    let short_description = get_short_description().await;
    let long_description = get_long_description().await;
    let closed = closes().await;

    let finished_short = format!("{type}: {desc}", type=commit_type, desc=short_description);
    let finished_description = format!(
        "{long_description}\n\nCloses: {closed}",
        long_description = long_description,
        closed = closed
    );

    Command::new("git")
        .args(["commit", "-m", &finished_short, "-m", &finished_description])
        .output()
        .await
        .expect("We had an issue committing your code.");

    return true;
}
