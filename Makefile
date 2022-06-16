build: 
	cargo build --features client,secrets,search
	cargo build --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets,search
	cd examples/cloudflare && cargo install -q worker-build && worker-build

build-release: 
	cargo build --features client,secrets,search --release
	cargo build --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets,search --release
	cd examples/cloudflare && cargo install -q worker-build && worker-build --release

check:
	cargo clippy --all-targets --features client,secrets,search
	cargo clippy --all-targets --target wasm32-unknown-unknown --no-default-features --features builders,workers,secrets,search





