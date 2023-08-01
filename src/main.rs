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
    /// Name of the repo your need
    repo: String,

    /// Name of the host
    host: Option<String>,

    /// Specify tag or branch
    checkout: Option<String>,
}

fn main() {
    let args = Args::parse();

    // HashMap to store the entered args
    let mut info_map: HashMap<&String, Vec<Option<String>>> = HashMap::new();
    let mut info_vec: Vec<Option<String>> = Vec::new();

    // Update the info vector with author, host and checkout info
    info_vec.push(args.host);
    info_vec.push(args.checkout);

    // Insert repo name as key with info vector as value
    info_map.insert(&args.repo, info_vec);

    // Git uname fetch
    let mut git_uname = Command::new("git");

    git_uname.arg("config").arg("--global").arg("user.name");

    let git_uname = git_uname.stdout(Stdio::piped())
        .output()
        .unwrap();

    let mut stdout = String::from_utf8(git_uname.stdout).unwrap();

    stdout.pop();

    // Git pull your repo
    let git_ssh_com: String = "git@github.com:".to_string();
    let repo: String = args.repo.to_string();

    let final_com = format!("{}{}/{}.git", git_ssh_com, stdout, repo);

    let mut gegit_com = Command::new("git");

    gegit_com.arg("clone").arg(final_com);
    gegit_com.output().unwrap();
}
