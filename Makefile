.PHONY: dev-api dev-web wasm test build
dev-api:
	cargo run -p server
dev-web:
	cd apps/web && npm run dev
wasm:
	wasm-pack build crates/game-core --target web --release --out-dir ../../apps/web/public/wasm --out-name game_core
test:
	cargo test --workspace
build: wasm
	cd apps/web && npm run build
	cargo build -p server --release

