![gitclean](https://github.com/Girgetto/gitclean/assets/33903092/19280ae3-2877-4631-9a63-bfe7ce9657b5)

# gitclean

`gitclean` is a command-line utility written in Rust, designed to help developers and system administrators clean up unused `.git` directories. It provides an interactive interface to navigate through directories starting from where you initialize the `gitclean` command. You can then choose which `.git` folders to delete using the arrow keys and the 'd' key.

## Features

- **Interactive Navigation**: Easily navigate through your file system starting from the current directory.
- **Safe Deletion**: Preview and select `.git` directories before deciding to delete them.
- **Efficient Clean-up**: Quickly free up space and declutter your development environment.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) 1.70 or later (includes `cargo`)

## Usage

### Run from source

Clone the repository and run directly with Cargo:

```bash
git clone https://github.com/Girgetto/gitclean.git
cd gitclean
cargo run --release
```

`gitclean` starts in the **current working directory**, so `cd` to the root you want to scan before running it.

### Install globally

Install the binary into `~/.cargo/bin` so it is available anywhere on your `PATH`:

```bash
cargo install --path .
```

Then launch it from any directory:

```bash
cd ~/projects
gitclean
```

### Download the pre-built binary

Download the latest binary from the [releases page](https://github.com/Girgetto/gitclean/releases/tag/v0.1.0).