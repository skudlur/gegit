/*
 *
 *  Pulling your repos made ez
 *
 */

 use clap::Parser;
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
 
     // Git uname fetch
     let mut git_uname = Command::new("git");
 
     git_uname.arg("config").arg("--global").arg("user.name");
 
     let git_uname = git_uname.stdout(Stdio::piped())
         .output()
         .unwrap();
 
     let mut stdout = String::from_utf8(git_uname.stdout).unwrap();
 
     stdout.pop();

     // Git pull your repo
     let host = args.host.unwrap_or("github".to_string());
     let checkout = args.checkout.unwrap_or("main".to_string());
     let repo: String = args.repo.to_string();
     let git_ssh_com = format!("git@{}.com:", host).to_string();

     let final_com = format!("{}{}/{}.git", git_ssh_com, stdout, repo);
     println!("{}", final_com);
 
     let mut gegit_com = Command::new("git");
 
     gegit_com.arg("clone").arg("-b").arg(checkout).arg(final_com);
     gegit_com.output().unwrap();

     // Checking if the repo was cloned successfully
    let (command, arg) = if cfg!(target_os = "windows") {
        ("dir", "/B")
    } else {
        ("ls", "")
    };

    let op = Command::new(command).arg(arg).output().unwrap();

    if op.status.success() {
        println!("Repo {} cloned successfully", repo);
    } else {
        println!("Check if repo name and branch name are correct");
    }

 }
 