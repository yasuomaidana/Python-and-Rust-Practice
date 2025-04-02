create_lib:
	@echo "Creating lib"
	pip uninstall digits_pi -y
	cd digits-pi && maturin develop
run_test:
	@echo "Running all Python tests"
	python -m unittest discover
run_python_from_rust:
	@echo "Running Python from Rust"
	cd py-rs-call && PYTHONPATH=../.venv/lib/python3.12/site-packages cargo run --
	cd py-rs-call && PYTHONPATH=../.venv/lib/python3.12/site-packages cargo test --color=always