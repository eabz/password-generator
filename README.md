# Easy to remember and readable password generator

This code is based on and ported to Rust from the password generator created by Christian Haensel and modified by Josh Hartman, which can be found at https://www.warpconduit.net/password-generator/.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Install

1. Install the crate

```
cargo install pwdgen
```

2. Run the program

```
pwdgen
```

## Build

1. Clone the repository

```
git clone https://github.com/eabz/pwdgen && cd pwdgen
```

2. Build the program

```
cargo build --release
```

3. Run the generator

```
./target/release/pwdgen
```

## Program flags

| Flag            | Default | Purpose                                 |
| --------------- | :-----: | --------------------------------------- |
| -p, --passwords |   10    | Amount of passwords to generate.        |
| -l, --length    |   14    | Amount of characters for each password. |
