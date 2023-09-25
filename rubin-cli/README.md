# Rubin CLI

Command-line interface for the Rubin library.

## Install

You can install the Rubin CLI using `cargo`:

```bash
cargo install rubin-cli
```

## Usage

The Rubin CLI offers two options:

* Creating a Rubin server which offers in-memory storage for key-value pairs (see [here](https://github.com/Tyrannican/rubin/tree/main/rubin))

* A CLI for interacting with a running Rubin server

### Rubin Server

```bash
Start a Rubin server on a given address / port

Usage: rubin server [OPTIONS]

Options:
  -a, --address <ADDRESS>  Server address to use [default: 127.0.0.1]
  -p, --port <PORT>        Server port to use [default: 9876]
  -h, --help               Print help
  -V, --version            Print version
```

### Rubin CLI

```bash
Start the CLI to interact with a Rubin server on a given address / port

Usage: rubin cli [OPTIONS]

Options:
  -a, --address <ADDRESS>  Server address to use [default: 127.0.0.1]
  -p, --port <PORT>        Server port to use [default: 9876]
  -h, --help               Print help
  -V, --version            Print version
```

### CLI Commands

* String commands:
    * `set [KEY] [VALUE]`: Adds a string value with a given key to the store
    * `get [KEY]`: Gets a string value from the store with a given key
    * `rm [KEY]`: Removes a string value from the store with the given key
    * `clr`: Clears all values in the string store, removing the keys as well
    * `incr`: Increment a value in the store by one
    * `decr`: Decrement a value in the store by one
    * `dump`: Dump the store out to the server's disk. Folder needs to exist.
* `exit`: Quit the CLI 
