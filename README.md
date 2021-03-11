# CodeOnly

Annoyed by comments cluttering your code?

Those stupid `//TODO` and `//FIXME` your co-worker put everywhere need to be removed?

### CodeOnly helps you out: no comments - no problems.

## Build

- set toolchain to nightly

```fish
rustup override set nightly
```

- run with:

```fish
cargo run [directory]
```

- install and run from anywhere:
```fish
cargo install --path .
~/.cargo/bin/codeonly [directory]
```

## Contribute

- add another `/src/codeonly_<file-extension>.rs` file that removes the comments from the given programming language
- import the module in `/src/main.rs`
- add another pattern match in `/src/main.rs`
