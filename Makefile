.PHONY: clean run test publish update
all: $(TARGET)

update: $(TARGET)
	cargo update

$(TARGET): $(SRC)
	cargo build

clean:
	cargo clean

run: $(TARGET)
	cargo run

test: $(TARGET)
	cargo test

publish: $(TARGET)
	cargo publish

release: $(TARGET)
	cargo build --release

