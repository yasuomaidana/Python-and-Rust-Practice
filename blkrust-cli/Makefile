build_image:
	@echo "Building image..."
	docker build -t blk-rust .

dev:
	DOCKER_BUILDKIT=1 docker build -t blk-rust .
	@echo "Building project..."
	docker run --name blk-rust-container -it --rm -v $(PWD):/app -w /app blk-rust /bin/sh
	# --rm deletes the container after it's done running
	#docker run --name blk-rust-container -it -v $(PWD):/app -w /app blk-rust /bin/sh

build_release:
	cargo fmt
	cargo clippy -- -D warnings # -D warnings means show all warnings as errors
	cargo check --release
	DOCKER_BUILDKIT=1 docker build -t blk-rust-release -f release.dockerfile .

run_release: build_release
	docker run --name blk-rust-release-container -it --rm blk-rust-release

build_linux:
	DOCKER_BUILDKIT=1 docker build -t blk-rust-linux -f release-linux.dockerfile .

run_linux: build_linux
	DOCKER_BUILDKIT=1 docker run --name blk-rust-linux-container -it --rm blk-rust-linux

build_linux_keep:
	DOCKER_BUILDKIT=1 docker build -t keep-blk-rust-linux -f keep.dockerfile .
	docker run --name keep-blk-rust-linux-container keep-blk-rust-linux
	docker cp keep-blk-rust-linux-container:/app/target/aarch64-unknown-linux-musl/release/blkrust-cli ./output/
	docker rm keep-blk-rust-linux-container
	docker image rm keep-blk-rust-linux
	
	