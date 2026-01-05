.DEFAULT_GOAL := build

build: 
	cargo build --release
	mv target/release/mdbook-sspaeti /usr/local/bin/mdbook-sspaeti


build-linux: 
	cargo build --release
	mv target/release/mdbook-sspaeti ~/.local/bin/mdbook-sspaeti

# build: prepare run

