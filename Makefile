.PHONY: all wasm ui clean

all: wasm ui

wasm:
	@echo "Building Wasm Engine..."
	cd soku_wasm && wasm-pack build --target web --out-dir ../soku_ui/src/wasm

ui:
	@echo "Building Frontend..."
	cd soku_ui && npm install && npm run build

clean:
	@echo "Cleaning up..."
	cd soku_core && cargo clean
	cd soku_wasm && cargo clean
	rm -rf soku_ui/dist
	rm -rf soku_ui/src/wasm
