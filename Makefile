c_python_image:
	docker build -t python_rust_practice_image -f dockerfile .
start_python_container:
	#docker run -it --rm --name python_rust_practice_container python_rust_practice_imag	
	# --rm: remove container after exit
	docker run -it --name python_rust_practice_container python_rust_practice_image