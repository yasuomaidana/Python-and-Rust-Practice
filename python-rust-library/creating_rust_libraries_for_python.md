# Creating Rust's libraries for Python

## Develop Rust's libraries for Python

1. Move to the directory where you want to create the Rust library.
2. Create a new Rust library with the following command: `maturin develop`
3. Go back to your Python project and load the Rust library with name defined in your cargo file

## Build Rust's libraries for Python

1. Move to the directory where you want to create the Rust library.
2. Create a new Rust library with the following command: `maturin build`
3. Install the generated wheel file with the following command: `pip install <wheel_file>`