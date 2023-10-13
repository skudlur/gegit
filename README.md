# gegit
gegit -> Getting your personal Git repositories from your terminal made easy

## Installing Rust
The project needs the rust compiler to be installed.
### Unix-like OS (Linux, macOS)
Run the following command:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Windows
Run the following command inside the gegit repo after cloning:
```bash
rustup-init.exe
```

# How to use
- Install gegit using the following command and then type the next command in your terminal.

```bash
cargo install gegit
```

```bash
gegit <your_personal_repo_remote_name>
```

**Note** -> Make sure you have your ssh setup and you have updated your username in the gitconfig.
The username in gitconfig should match your host username. You can check your gitconfig username using the following command:
```bash
git config --global user.name
```
In case it is different, change gitconfig username using the following command:
```bash
git config --global user.name <your host username>
```

#### Contributor Name: Suhas K Viswanath
