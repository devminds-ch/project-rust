# Rust Training Project by [devminds GmbH](https://devminds.ch)

This Rust project is used for trainings offered by devminds GmbH.

The project contains an application providing a CLI to calculate the sum of two numbers.

```
Rust training project by devminds GmbH
This Rust project is used for trainings offered by devminds GmbH.
The project contains an application providing a CLI to calculate the sum of two numbers.

Usage: rust_training_project <COMMAND>

Commands:
  sum   Show the sum of two numbers on the console
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


## Build and test instructions

### Build Rust application

Build the Rust application debug profile:

```bash
cargo build
```

Build the Rust application release profile:

```bash
cargo build --profile release
```

### Build application documentation

Build the application documentation:

```bash
cargo doc --no-deps --profile release
```

### Run Rust static analysis

Check if source code is formatted properly:

```bash
cargo fmt --all -- --check
```

Run [Clippy](https://github.com/rust-lang/rust-clippy):

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Run Rust tests

Run the Rust tests:

```bash
cargo test
```
