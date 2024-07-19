OUT_NAME=gameplay
OUT_DIR=./wasm

build:
	@cd rust/export && cargo build && wasm-pack build --release --out-name $(OUT_NAME) --out-dir ../../$(OUT_DIR)
