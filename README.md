# Go Rusty JSON Bytes
Go marshalling uses reflection, so it's slow for large amounts of data. Inspired by https://www.reddit.com/r/golang/comments/1flap0d/very_slow_json_marshalling_what_do_you_guys_do/

## Prerequisites
1. [Go 1.23.1](https://go.dev/doc/install)
2. [Rust v1.8.1](https://www.rust-lang.org/tools/install)

## Building
### Build the Rust library
1. `cd rusty-json-bytes`
2. `cargo build --release`

## Running
### Run the Go test program
1. `CGO_ENABLED=1 go run go-json-bytes.go`

## Implementing in your Go program
