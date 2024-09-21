# Go Rusty JSON Bytes
Go marshalling uses reflection, so it's slow for large amounts of data. Inspired by https://www.reddit.com/r/golang/comments/1flap0d/very_slow_json_marshalling_what_do_you_guys_do/

## Prerequisites
1. [Go 1.23.1](https://go.dev/doc/install)
2. [Rust v1.8.1](https://www.rust-lang.org/tools/install)
3. If building for Linux (**untested**):
   1. `rustup target add x86_64-unknown-linux-gnu`
   2. `sudo apt-get install build-essential`

## Building
### Build the Rust library
1. `cd rusty-json-bytes`
2. `cargo build --release`
3. Copy the executable for your platform to the root directory, so that it lives at _the same level_ as your Go program.
   1. e.g. Windows `cp target/release/rusty_json_bytes.dll ..`
   2. e.g. Linux `cp target/release/rusty_json_bytes.so ..`

## Running
### Run the Go test program
```
$ CGO_ENABLED=1 go run go-json-bytes.go

Bytes from Rust: [123 34 102 105 101 108 100 49 34 58 91 49 48 49 44 49 50 48 44 57 55 44 49 48 57 44 49 49 50 44 49 48 56 44 49 48 49 44 51 50 44 49 48 48 44 57 55 44 49 49 54 44 57 55 93 44 34 102 105 101 108 100 50 34 58 49 50 51 125]
JSON from Rust: {"field1":[101,120,97,109,112,108,101,32,100,97,116,97],"field2":123}
```


## Implementing in your Go program
Follow the example in `main()` of `go-json-bytes.go`.

Note that before calling out to this library, you must have:
```go
/*
// Link the Rust shared library that we will call from Go
#cgo LDFLAGS: -L./ -lrusty_json_bytes

#include <stdlib.h> // Include the standard C library (for memory management)
#include <stdint.h> // Include stdint.h for uint8_t
#include <stddef.h> // Include stddef.h for size_t

typedef struct {
    void* ptr;
    size_t len;
} ByteReturn;

// Declare the Rust functions
extern ByteReturn marshal_bytes(const uint8_t* field1, size_t field1_len, int field2);
extern void free_bytes(void* ptr);
*/
import "C"
```
in your Go code.

While it's in a comment block, it's interpreted by Go. You need to add all necessary Rust functions and structs that you plan on using in your Go program.