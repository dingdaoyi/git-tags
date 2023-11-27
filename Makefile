

build-linux: $(PYTHON_FILES)
	cargo build --release --target=x86_64-unknown-linux-musl

