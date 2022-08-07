use crate::commands::Commands;
use crate::utils;

use colored::Colorize;
use console;
use dialoguer::{theme, FuzzySelect, MultiSelect};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn filter_choice_cli(choices: String) -> Result<Commands, String> {
    let items = choices.split_terminator("\n").collect::<Vec<_>>();
    let selection = FuzzySelect::with_theme(&theme::ColorfulTheme::default())
        .items(&items)
        .with_prompt("Please choose a menu:")
        .default(1)
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't fuzzy search");

    match selection {
        Some(index) => Commands::from_string(items[index].to_string()),
        None => Err("No item was selected!".green().to_string()),
    }
}

pub fn choice_no_limit(
    mut choices: Vec<String>,
    has_none: bool,
) -> Result<Option<Vec<String>>, String> {
    if has_none {
        choices.push("None".to_string());
    }

    let selected: Option<Vec<usize>> = MultiSelect::new()
        .items(&choices)
        .with_prompt("Please choose files to stage")
        .interact_on_opt(&console::Term::stderr())
        .expect("Couldn't make a choice");

    match selected {
        None => Err("No items were selected".to_string()),
        Some(indexes) => {
            let mut all_choices = Vec::<String>::new();
            all_choices.reserve(indexes.len());
            for i in indexes.into_iter() {
                all_choices.push(utils::strip_colors(choices[i].to_string()).to_owned());
            }
            if all_choices.contains(&choices[choices.len() - 1].to_string()) {
                Ok(None)
            } else {
                Ok(Some(all_choices))
            }
        }
    }
}

pub fn git_add(input: Vec<String>) -> Result<(), String> {
    let git_add_cmd = Command::new("git")
        .arg("add")
        .args(input)
        .spawn()
        .expect("Couldn't run `git add`");

    let output = git_add_cmd.wait_with_output().unwrap();

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    } else {
        Ok(())
    }
}

pub fn git_reset(input: Vec<String>) -> Result<(), String> {
    Ok(())
}

pub fn git_status_short() -> Result<Option<String>, String> {
    let git_status_cmd = Command::new("git")
        .arg("status")
        .arg("--short")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Couldn't call git add!");

    let git_status = git_status_cmd.wait_with_output().unwrap();

    if git_status.status.success() {
        let status_output = String::from_utf8_lossy(&git_status.stdout).to_string();
        if status_output == "" {
            Ok(None)
        } else {
            Ok(Some(status_output))
        }
    } else {
        Err(String::from_utf8_lossy(&git_status.stderr).to_string())
    }
}
