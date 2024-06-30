build-bot:
	cd crates/my_bot && cargo build

build-train:
	cargo build --package train

build: build-train build-bot