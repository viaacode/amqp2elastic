# Amqp2Elastic

## Prerequisites

- Rust toolchain: see [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- Cargo (should be installed along with the Rust toolchain)

Test the installation with `rustc --version`. (If you get a `command not
found`, either restart the terminal and/or add the  `~/.cargo/bin` directory to
your `PATH`.)

## Usage

- Clone this repository.
- Fill in and source an `.env` file (Example in `.env.example`):

```bash
$ export $(cat .env.qas | xargs)
```

- Run the tests with `cargo test`.
- Run with `cargo run`.
