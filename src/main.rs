use std::{path::PathBuf, process::Command};

use clap::Parser;
use url::Url;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repo_url: String,
}

fn main() {
    let args = Args::parse();

    let url = Url::parse(&args.repo_url).expect("Failed to parse repo URL");

    assert_eq!(url.scheme(), "https");
    assert_eq!(url.domain(), Some("github.com"));
    assert_eq!(url.port(), None);
    assert_eq!(url.password(), None);
    assert_eq!(url.username(), "");
    assert_eq!(url.query(), None);

    let path = url
        .path_segments()
        .expect("Failed to get path segments")
        .collect::<Vec<_>>();

    assert_eq!(path.len(), 2);

    let [user, repo] = [path[0], path[1]];

    assert!(!user.is_empty());
    assert!(!repo.is_empty());

    // Check if the directory already exists
    let relative_path = PathBuf::from(user);
    if !relative_path.exists() {
        // Create it if it doesn't
        println!("Creating directory: {:?}", relative_path);
        std::fs::create_dir(&relative_path).expect("Failed to create directory");
    }

    // Move into the parent directory for the repo
    let full_path = std::env::current_dir()
        .expect("Failed to get current directory")
        .join(relative_path);
    std::env::set_current_dir(full_path).expect("Failed to set current directory");

    // Call git clone with the repo URL
    let status = Command::new("git")
        .args(["clone", &args.repo_url])
        .status()
        .expect("Failed to clone repo");

    std::process::exit(status.code().unwrap_or(1));
}
