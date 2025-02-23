# Python-and-Rust-Practice

## Defining Python Containers

The `dockerfile` contains instructions to create a Docker image for a Python environment. Here is a brief explanation of
each line:

1. `FROM python:3.10-alpine`: This line specifies the base image for the Docker image. It uses the official Python 3.10
   image based on Alpine Linux, which is a lightweight Linux distribution.
2. `RUN apk add --no-cache util-linux`: This line installs the `util-linux` package using the Alpine package manager (
   `apk`). The `--no-cache` option ensures that no cache is used, reducing the image size. The `util-linux` package
   includes various useful utilities, such as `lsblk`.

This `dockerfile` sets up a minimal Python environment with additional utilities for use in a container.

## Accessing to the Python Container

To access the Python container, you can run the following command:

```bash
docker exec -it <mycontainer> sh
```

[//]: # (## Activate container Python environment)

## Logging levels

| Level    |  Numeric value  |
|----------|:---------------:|
| CRITICAL |       50        |
| ERROR    |       40        |
| WARNING  |       30        |
| INFO     |       20        |
| DEBUG    |       10        |
| NOTSET   |        0        |

| Level     | When it’s used                                                                                       |
|-----------|------------------------------------------------------------------------------------------------------|
| DEBUG     | Detailed information, typically of interest only when diagnosing problems.                           |
| INFO      | Confirmation that things are working as expected.                                                    |
| WARNING   | An indication that something unexpected happened, or indicative of some problem in the near future.  |
| ERROR     | Due to a more serious problem, the software has not been able to perform some function.              |
| CRITICAL  | A serious error, indicating that the program itself may be unable to continue running.               |