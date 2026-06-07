# Rust Port Scanner

A fast multithreaded TCP port scanner built in Rust.

## Overview

This project is a command-line tool that scans TCP ports on a target host using multiple threads. It demonstrates concurrent programming, command-line argument parsing, networking concepts, and inter-thread communication in Rust.

The scanner distributes the work across several threads to improve scanning speed while maintaining a simple and lightweight architecture.

## Features

* TCP port scanning
* IPv4 and IPv6 support
* Multithreaded execution
* Configurable number of worker threads
* Command-line interface
* Channel-based communication between threads
* Error handling for invalid arguments

## Technologies

* Rust
* std::net
* std::thread
* std::sync::mpsc

## Usage

Scan using the default thread count:

```bash
cargo run -- 192.168.1.1
```

Scan using a custom number of threads:

```bash
cargo run -- -j 100 192.168.1.1
```

Display help:

```bash
cargo run -- -h
```

## Example Output

```text
Scanning 192.168.1.1 ...

Port 22 open
Port 80 open
Port 443 open
```

## What I Learned

This project helped me practice:

* Concurrent programming
* Thread management
* Message passing with channels
* Network programming
* Command-line application development
* Error handling
* Rust ownership and borrowing

## Project Structure

```text
src/
 ├── main.rs
```

## Roadmap

### Version 1.0

* [x] Basic TCP port scanning
* [x] Multithreading support
* [x] Argument parsing
* [x] IPv4 support

### Version 1.1

* [ ] Display scan progress
* [ ] Better error messages
* [ ] Scan result summary
* [ ] Configurable timeout

### Version 1.2

* [ ] IPv6 improvements
* [ ] Export results to JSON
* [ ] Export results to CSV
* [ ] Logging support

### Version 2.0

* [ ] Service detection
* [ ] Banner grabbing
* [ ] Host discovery
* [ ] CIDR range scanning
* [ ] Async implementation with Tokio

## Future Improvements

* Replace manual argument parsing with Clap
* Improve performance using async I/O
* Add integration tests
* Create release binaries for Linux, Windows and macOS

## License

MIT License
