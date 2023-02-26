# Rust

- install
```sh 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

To configure your current shell, run:
```sh
source "$HOME/.cargo/env"
```

- [rustup book](https://rust-lang.github.io/rustup/)


- Create new project
```sh 
cargo new hello_cargo
cargo build
cargo run
cargo check
cargo clean
```