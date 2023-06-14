.DEFAULT_GOAL := build

build: 
	cargo build --release
	mv target/release/mdbook-sspaeti /usr/local/bin/mdbook-sspaeti

# build: prepare run

