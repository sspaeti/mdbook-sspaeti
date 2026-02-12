.DEFAULT_GOAL := build-linux

build: 
	cargo build --release
	mv target/release/mdbook-sspaeti /usr/local/bin/mdbook-sspaeti


build-linux: 
	cargo build --release
	mv target/release/mdbook-sspaeti ~/.local/bin/mdbook-sspaeti

# build: prepare run

