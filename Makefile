format:
	cargo fmt
lint:
	cargo clippy
build:
	cargo build
test:
	cargo test
run:
	cargo run --bin bioinfo_tool
doc:
	cargo doc
clean:
	cargo clean
all: format lint build test run doc clean