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

    // let selections: Vec<String> = branches.into_iter()
    //     .map(|branch| branch.unwrap().0.name().unwrap().unwrap().to_string())
    //     .collect();

    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", selections[selection]);
}
