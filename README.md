# A lolcat implementation, written in rust
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/BlueTufa/lolcat/Rust)
![GitHub issues](https://img.shields.io/github/issues/BlueTufa/lolcat)
![GitHub pull requests](https://img.shields.io/github/issues-pr/BlueTufa/lolcat)

A simple cat utility which outputs lols.

Works with either an input file name or stdin.  

## How to install
### Clone this repository.
```bash
git clone https://github.com/BlueTufa/lolcat.git
```
### Install rustup.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
> Alternate (MacOS Intel): 
```bash
brew install rust
```
Then do a cargo install within the root of this source tree:
```bash
cargo install --path $(pwd)
```

Make sure you have `$HOME/.cargo/bin` in your path.
```bash
export PATH=$HOME/.cargo/bin:$PATH
```

# Usage:
```bash
lolcat filename
```

```bash
somecommand | lolcat
```
