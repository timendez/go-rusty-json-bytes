package main

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

import (
	"fmt"
)

func main() {
	// Example data to send to the Rust function
	field1 := []byte("example data") // Example byte slice
	field2 := 123                    // Example integer

	// Call the Rust function marshal_bytes to generate JSON bytes
	result := C.marshal_bytes((*C.uint8_t)(&field1[0]), C.size_t(len(field1)), C.int(field2))

	// Convert the C bytes returned by Rust to a Go byte slice
	jsonBytes := C.GoBytes(result.ptr, C.int(result.len))
	fmt.Println("JSON Bytes from Rust:", jsonBytes)

	// Convert the byte slice to a Go string for easier printing
	jsonString := string(jsonBytes)

	// Print the JSON string generated by the Rust code
	fmt.Println("JSON from Rust:", jsonString)

	// Important: Call free_bytes to free the memory allocated by Rust for the JSON bytes
	C.free_bytes(result.ptr)
}
