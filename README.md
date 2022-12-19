# atcoder-rust

- <https://github.com/qryxip/cargo-compete>

## init

```fish
cargo install cargo-compete
cargo compete login
alias ac='cargo compete login atcoder'
```

## usage

```fish
read contest # abc123
ac new $contest
cd $contest
ac open
code -r .
idea .
ac submit a
```
