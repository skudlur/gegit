# gegit
gegit -> Command line tool that helps you clone a said repository to your system in one step with the help of SSH and if it is unable to find your username, it prompts you for it. If SSH is not set up, it prints an error message and exits.  If it is unable to find your username, it prompts you for it. It is written in Rust and uses the cargo package manager for installation.

You can also use gegit to clone a repository from a different user's GitHub account or to even clone a specific branch or tag of a repository.
# How to use
- To use gegit, first install it using the following command:
```bash
cargo install gegit
```
- Once gegit is installed, you can use it to clone your personal Git repositories by running the following command:
```bash
gegit <your_personal_repo_remote_name>
```

- To clone a repository from a different user's GitHub account, specify the owner of the repository using the owner option:
```bash
gegit <repo_name> --owner <owner_name>
```

- To clone a specific branch or tag of a repository, specify the branch or tag using the checkout option:
```bash
gegit <repo_name> --checkout master
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

To clone a repository from a different user's GitHub account, say john-doe run the following command:
```bash
gegit my-repo --owner john-doe
```

To clone a specific branch or tag of a repository, say the master branch,  run the following command:
```bash
gegit my-repo --checkout master
```

It's that easy.

**Note** -> Make sure you have your SSH set up and you have updated your username in the gitconfig.

#### Contributor Name: Suhas K Viswanath