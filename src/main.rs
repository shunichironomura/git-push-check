use clap::{Parser, ValueEnum};
use git2::Repository;
use log::{debug, error};
use std::process;

/// Check if HEAD is pushed to a remote.
///
/// This application checks if the current HEAD commit is reachable via a remote branch
/// or tag. You can limit the check to branches, tags, or all references.
/// You may want to fetch updates before running this application.
/// If the HEAD commit is found in a remote branch or tag, the application exits with status 0.
/// If the HEAD commit is not found in any remote reference, the application exits with status 2.
/// If an error occurs, the application exits with status 1.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Which references to check: branches, tags, or all (default: all)
    #[arg(long, value_enum, default_value_t = Only::All)]
    only: Only,
}

#[derive(ValueEnum, Debug, Clone, PartialEq, Eq)]
enum Only {
    Branches,
    Tags,
    All,
}

fn main() {
    env_logger::init();

    let args = Cli::parse();
    debug!("Parsed arguments: {args:?}");

    match run(&args.only) {
        Ok(true) => process::exit(0),
        Ok(false) => process::exit(2),
        Err(e) => {
            error!("Error: {e}");
            process::exit(1);
        }
    }
}

fn run(only: &Only) -> Result<bool, Box<dyn std::error::Error>> {
    // Open the repository from the current directory or an ancestor.
    let repo = Repository::open_from_env()?;
    debug!("Opened repository: {:}", repo.path().display());

    // Get the current commit (HEAD)
    let head_ref = repo.head()?;
    let head_commit = head_ref.peel_to_commit()?;
    let current_oid = head_commit.id();

    let mut pushed = false;

    // Check remote branches if requested.
    if *only == Only::Branches || *only == Only::All {
        for reference in repo.references()? {
            let reference = reference?;
            if let Some(name) = reference.name()
                && name.starts_with("refs/remotes/")
                && let Ok(remote_commit) = reference.peel_to_commit()
            {
                // Check if the remote branch tip equals HEAD or contains HEAD in its history.
                if remote_commit.id() == current_oid
                    || repo.graph_descendant_of(remote_commit.id(), current_oid)?
                {
                    println!(
                        "Local HEAD ({current_oid}) is pushed (found in remote branch: {name}).",
                    );
                    pushed = true;
                    break;
                }
            }
        }
    }

    // If not found via branches and checking tags is enabled, check tags.
    if !pushed && (*only == Only::Tags || *only == Only::All) {
        for reference in repo.references()? {
            let reference = reference?;
            if let Some(name) = reference.name()
                && name.starts_with("refs/tags/")
            {
                // For annotated tags, peel to the commit they reference.
                if let Ok(tag_commit) = reference.peel_to_commit()
                    && tag_commit.id() == current_oid
                {
                    println!("Local HEAD ({current_oid}) is pushed (found in remote tag: {name}).",);
                    pushed = true;
                    break;
                }
            }
        }
    }

    if pushed {
        Ok(true)
    } else {
        println!("Local HEAD ({current_oid}) is NOT pushed to remote.");
        Ok(false)
    }
}
