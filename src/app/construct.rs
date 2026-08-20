use std::vec;

use clap::builder::Str;
use dialoguer::FuzzySelect;
use owo_colors::OwoColorize;

use crate::app::app_config;

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

    return item_strings[selected_type]
        .split(":")
        .next()
        .expect("Shouldn't be possible to get rid of the colon.")
        .to_owned();
}

pub async fn construct_message(settings: app_config::Settings) -> bool {
    let commit_type = get_commit_type(settings).await;

    return true;
}
