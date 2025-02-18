c_python_image:
	docker build --build-arg INSTALL_MODE=develop -t python_rust_practice_image -f dockerfile .
c_python_image_prod:
	docker build --build-arg -t python_rust_practice_image .
start_python_container:
	#docker run -it --rm --name python_rust_practice_container python_rust_practice_imag	
	# --rm: remove container after exit
	docker run -it --name python_rust_practice_container python_rust_practice_image