build_image:
	@echo "Building image..."
	docker build -t blk-rust .

build:
	@echo "Building project..."
	docker run --name blk-rust-container -it --rm -v $(PWD):/app -w /app blk-rust /bin/sh
	# --rm deletes the container after it's done running
	#docker run --name blk-rust-container -it -v $(PWD):/app -w /app blk-rust /bin/sh

build_release:
	docker run --rm -v $(PWD):/app -w /app blk-rust cargo build --release
