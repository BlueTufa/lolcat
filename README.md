# A lolcat implementation, written in rust

A simple cat utility which outputs lols.

Works with either an input file name or stdin.  

## How to install
Start by installing rustup.
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
