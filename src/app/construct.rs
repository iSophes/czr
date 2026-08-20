use std::vec;

use dialoguer::FuzzySelect;
use owo_colors::OwoColorize;

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
            .into_string()
            .unwrap();

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

        item_strings.push(converted_commit_type);
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

    let commit_type = item_strings[selected_type]
        .split(":")
        .next()
        .expect("Shouldn't be possible to get rid of the colon.")
        .to_owned()
        .split(" ")
        .nth(1)
        .expect("Shouldn't be possible to get rid of the space.")
        .to_owned();

    let emoji_code = settings.commit_codes.get(&commit_type).unwrap();

    return format!("{code} {type}", code=emoji_code, type=commit_type);
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
        .interact_text()
        .unwrap();

    return closes;
}

pub async fn construct_message(settings: app_config::Settings) -> bool {
    let commit_type = get_commit_type(settings).await;
    let short_description = get_short_description().await;
    let long_description = get_long_description().await;
    let closed = closes().await;

    return true;
}
