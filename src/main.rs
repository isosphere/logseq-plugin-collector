use clap::Parser;
use git2::{build::CheckoutBuilder, build::RepoBuilder, BranchType, Repository, FetchOptions};
use serde::Deserialize;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

const LOGSEQ_MARKETPLACE_REPO_URL: &str = "https://github.com/logseq/marketplace";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PluginManifest {
    title: String,
    description: String,
    author: String,
    /// Github username/repo
    repo: String, 
    icon: Option<String>,
    theme: Option<bool>,
    sponsors: Option<Vec<String>>,
    effect: Option<bool>,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The destination (directory) to write the output to
    #[arg(short, long)]
    destination: String,

    /// If true, the destination will be created if not found
    #[arg(short, long, default_value_t = true)]
    create_destination: bool,
}


fn clone_or_update_repo(path: &Path, url: &str) -> Result<Repository, ()> {
    match path.exists() {
        true => {
            println!("Repository already exists at {}, updating.", path.display());
            let repo = Repository::open(path).unwrap();
            let mut checkout_opt = CheckoutBuilder::new();
            checkout_opt.use_theirs(true).force();

            match repo.checkout_head(Some(&mut checkout_opt)) {
                Ok(_) => Ok(repo),
                Err(e) => {
                    println!("Error checking out head: {}; url: {}", e, url);
                    return Err(());
                }
            }
        }
        false => {
            println!("Cloning repository to {}", path.display());
            let mut fetch_options = FetchOptions::new();
            fetch_options.download_tags(git2::AutotagOption::None);
            fetch_options.depth(1);

            let mut repo = RepoBuilder::new();
            repo.fetch_options(fetch_options);

            match repo.clone(url, path) {
                Ok(repo) => Ok(repo),
                Err(e) => {
                    println!("Error cloning repository: {}, url: {}", e, url);
                    return Err(());
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let destination = Path::new(&args.destination);

    if !destination.exists() && args.create_destination {
        println!("Creating destination directory: {}", args.destination);
        std::fs::create_dir_all(destination).unwrap();
    } else if !destination.exists() {
        println!("Destination directory not found and we are configured to not create it: {}", args.destination);
        std::process::exit(1);
    }
    
    let marketplace_path = destination.join(".marketplace");
    clone_or_update_repo(&marketplace_path, LOGSEQ_MARKETPLACE_REPO_URL).unwrap();

    println!("Iterating over all marketplace plugins and cloning/updating their repositories...");

    let plugin_root = marketplace_path.join("packages");
    for entry in WalkDir::new(plugin_root) {
        match entry {
            Ok(entry) => {
                if entry.file_name() == "manifest.json" {
                    let manifest_path = entry.path();
                    let manifest_content = fs::read_to_string(manifest_path).unwrap();
                    let manifest: PluginManifest = serde_json::from_str(&manifest_content).unwrap();

                    let (username, repo) = manifest.repo.split_once("/").unwrap();

                    let repo_url = format!("https://github.com/{}/{}", username, repo);

                    let local_path = destination.join(repo);
                    clone_or_update_repo(&local_path, &repo_url).unwrap();
                }
            },
            Err(e) => {
                println!("Error iterating over directory: {}", e);
            }
        }
    }
}
