
CC=gcc
BIN=test

ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input.$(EXT)
	$(CC) test.c -L ./target/debug/ -ldouble_input -o $(BIN)
	LD_LIBRARY_PATH=./target/debug/ ./$(BIN)

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf $(BIN)
