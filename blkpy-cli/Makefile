c_python_image:
	docker build --build-arg INSTALL_MODE=develop -t python_rust_practice_image -f dockerfile .
c_python_image_prod:
	docker build --build-arg -t python_rust_practice_image .
dev:
	DOCKER_BUILDKIT=1 docker build --build-arg INSTALL_MODE=develop -t python_rust_practice_dev_image -f dev.dockerfile .
	#docker run -it --rm --name python_rust_practice_container python_rust_practice_imag	
	# --rm: remove container after exit
	#docker run -it --rm --name dev_container -v $(PWD):/app -w /app python_rust_practice_dev_image sh
	docker run -it --rm --name dev_container -v $(PWD):/app -w /app python_rust_practice_dev_image
install_dev:
	@pip show setuptools > /dev/null || pip install setuptools
	pip install -r requirements.txt
	# python setup.py develop old way
	python -m pip install -e .

build:
	# python setup.py --help-commands | grep wheel
	# python setup.py bdist_wheel old way
	python -m build --wheel

run_build:build
	DOCKER_BUILDKIT=1 docker build -t python-dist-image -f release.dockerfile .
	docker run -it --rm --name python-dist-container python-dist-image
	docker image rm python-dist-image
clean:
	rm *.log