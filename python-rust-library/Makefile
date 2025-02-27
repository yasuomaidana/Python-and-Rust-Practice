create_lib:
	@echo "Creating lib"
	pip uninstall digits_pi -y
	cd digits-pi && maturin develop
run_test:
	@echo "Running all Python tests"
	python -m unittest discover