use dialoguer::{theme::ColorfulTheme, Select};
use git2::{Repository, BranchType};

fn main() {
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
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
        println!("[No branches] Are you in a git repository?");
        std::process::exit(1);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Switch to:")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    let branch_name = selections[selection].as_str();
    match repo.set_head(&("refs/heads/".to_owned() + branch_name)) {
        Ok(_) => (),
        Err(e) => panic!("failed to change branch: {}", e),
    }
}
