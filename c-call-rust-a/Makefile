
CC=gcc
BIN=test
EXT=a

ifeq ($(shell uname),Darwin)
    LDFLAGS := -Wl,-dead_strip
else
    LDFLAGS := -Wl,-Bstatic -ldouble_input -Wl,-Bdynamic -lpthread -ldl	
endif


all: target/debug/libdouble_input.$(EXT)
	$(CC) test.c -L ./target/debug/ $(LDFLAGS) -o $(BIN)
	./$(BIN)

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
	rm -rf $(BIN)
