all: clean format build  test

build: $(SRC)
	cargo build

update:
	cargo update

format:
	cargo fmt
	cargo clippy

clean:
	cargo clean

test: build
	cargo test

test-release: release
	cargo test --release

publish: clean format update release
	cargo publish

release:
	cargo build --release

run:
	cargo run
