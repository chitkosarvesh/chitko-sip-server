# Chitko SIP Server

![Release](https://img.shields.io/github/v/release/chitkosarvesh/chitko-sip-server?label=Release&sort=semver)

## Description

Implementation of [RFC 3261](https://datatracker.ietf.org/doc/html/rfc3261) in Rust.
Initially planned support for SIP over TCP, with media over UDP.

## Features not in road map:

- TLS/DTLS
- SIPS
- RTSP

## Building instructions

### Prerequisites

- Rust 1.56.0+

### Running

```bash
cargo run
```

### Command line arguments

```bash
cargo run -- -h
```

### Build

Debug

```bash
cargo build
```

Release

```bash
cargo build --release
```