![gitclean](https://github.com/Girgetto/gitclean/assets/33903092/19280ae3-2877-4631-9a63-bfe7ce9657b5)

# gitclean

`gitclean` is a command-line utility written in Rust, designed to help developers and system administrators clean up unused `.git` directories. It provides an interactive interface to navigate through directories starting from where you initialize the `gitclean` command. You can then choose which `.git` folders to delete using the arrow keys and the 'd' key.

## Features

- **Interactive Navigation**: Easily navigate through your file system starting from the current directory.
- **Safe Deletion**: Preview and select `.git` directories before deciding to delete them.
- **Efficient Clean-up**: Quickly free up space and declutter your development environment.

## Getting Started

### Download the binary

Download the latest binary from the [releases page](https://github.com/Girgetto/gitclean/releases/tag/v0.1.0)

### Prerequisites

Ensure you have [Rust and Cargo](https://www.rust-lang.org/tools/install) installed.

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/gitclean.git
```

2. Build the project:

```bash
cargo build --release
```

3. Run the executable:

```bash
./target/release/gitclean
```

4. Follow the on-screen instructions to navigate through your file system and select `.git` directories to delete.
