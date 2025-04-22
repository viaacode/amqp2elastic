# Amqp2Elastic

Reads events from VRT as XML from a RabbitMQ-queue (`vrt2elk_events_xml_q`),
transforms them to JSON and sends them to ELK via the RabbitMQ-queue
`vrt2elk_events_json_q`.

## Prerequisites

- Rust toolchain: see [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).
- Cargo (should be installed along with the Rust toolchain)

Test the installation with `rustc --version`. (If you get a `command not
found`, either restart the terminal and/or add the  `~/.cargo/bin` directory to
your `PATH`.)

## Usage

- Clone this repository.
- Fill in and export the variables in a `.env` file (Example in
  `.env.example`):

```bash
$ export $(grep -v '^#' .env | xargs)
```

- Run the tests with `cargo test`.
- Run with `cargo run`.
- Or, export env-vars and run in one go:

```bash
$ export $(grep -v '^#' .env | xargs); ./amqp2elastic
```
