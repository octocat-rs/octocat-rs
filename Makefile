build: 
	cargo build --features client,secrets 
	cargo build --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets
	cd examples/cloudflare && cargo install -q worker-build && worker-build

build-release: 
	cargo build --features client,secrets --release
	cargo build --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets --release
	cd examples/cloudflare && cargo install -q worker-build && worker-build --release

check:
	cargo clippy --all-targets --features client,secrets
	cargo clippy --all-targets --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets

