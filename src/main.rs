/*
 *
 *  Pulling your repos made ez
 *
 */

use clap::Parser;
use std::collections::HashMap;
use std::process::{Command, Stdio};

/// Git your git repos quick
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address of the repo in the format uname/reponame
    repo_address: String,

    /// Specify tag or branch
    checkout: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Split the repo_address into uname and reponame
    let parts: Vec<&str> = args.repo_address.split('/').collect();
    if parts.len() != 2 {
        eprintln!("Invalid repo address. Use the format uname/reponame");
        std::process::exit(1);
    }

    let uname = parts[0];
    let reponame = parts[1];

    // Git uname fetch
    let mut git_uname = Command::new("git");

    git_uname.arg("config").arg("--global").arg("user.name");

    let git_uname = git_uname.stdout(Stdio::piped()).output().unwrap();

    let mut stdout = String::from_utf8(git_uname.stdout).unwrap();

    stdout.pop();

    // Git pull your repo
    let git_ssh_com: String = "git@github.com:".to_string();

    let final_com = format!("{}{}/{}.git", git_ssh_com, stdout, reponame);

    let mut git_clone_com = Command::new("git");

    git_clone_com.arg("clone").arg(final_com);
    
    if let Some(checkout) = args.checkout {
        git_clone_com.arg("--branch").arg(checkout);
    }

    git_clone_com.arg(format!("--single-branch"));

    git_clone_com.output().unwrap();
}