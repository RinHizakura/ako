all: build run-cli

build:
	cargo build

run-cli:
	cd cli; cargo run

clean:
	cargo clean
	cd cli; cargo clean
