use dialoguer::{theme::ColorfulTheme, Select};
use git2::{Repository, BranchType};
use std::process::Command;

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(_) => {
            println!("you must be in a git repository to successfully run this command.");
            std::process::exit(1);
        },
    };

    let branches = match repo.branches(Option::from(BranchType::Local)) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to load branches: {}", e),
    };

    let selections: Vec<String> = branches.into_iter()
        .map(|branch| {
            branch.unwrap().0.name().unwrap().unwrap().to_string()
        })
        .collect();

    if selections.len() == 0 {
        println!("[no branches] are you in a git repository?");
        std::process::exit(1);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("switch to:")
        .items(&selections[..])
        .interact()
        .unwrap();

    let branch_name = selections[selection].as_str();

    match Command::new("git")
        .args(&["checkout", branch_name])
        .output() {
        Ok(_) => (),
        Err(e) => panic!("failed to change branch: {}", e),
    }
}
