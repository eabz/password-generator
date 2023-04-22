# Easy to remember and readable password generator

This library contains a simple program to generate simple passwords.

This code is based and ported to rust from the password generator created by Christian Haensel and modified by Josh Hartman found in https://www.warpconduit.net/password-generator/

## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Install

1. Clone the repository

```
git clone https://github.com/eabz/password-generator && cd password-generator
```

2. Build the program

```
cargo build --release
```

3. Run the generator

```
./target/release/generator
```

## Program flags

|        Flag        | Default |                 Purpose                 |
| :----------------: | ------: | :-------------------------------------: |
| --passwords or --p |      10 |    Amount of passwords to generate.     |
|      --length      |      14 | Amount of characters for each password. |
