# gegit
gegit -> Command line tool that helps you clone a said repository to your system in one step. It is written in Rust and uses the cargo package manager for installation.

# How to use
- To use gegit, first install it using the following command:
```bash
cargo install gegit
```
Once gegit is installed, you can use it to clone your personal Git repositories by running the following command:
```bash
gegit <your_personal_repo_remote_name>
```


# Example
To clone your personal Git repository called my-repo, you would run the following command:
```bash
gegit my-repo
```

gegit will clone your repository to the current directory. You can then start working on your repository by running the following command:
```bash
cd my-repo
```
It's that easy.

**Note** -> Make sure you have your SSH set up and you have updated your username in the gitconfig.

#### Contributor Name: Suhas K Viswanath