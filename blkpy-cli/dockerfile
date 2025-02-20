FROM python:3.10-alpine

# Install lsblk
RUN apk add --no-cache util-linux

# Install dependencies
COPY requirements.txt .
COPY blkpy/ blkpy/
COPY lsblk_selector.py .
COPY setup.py .
RUN pip install -r requirements.txt

ARG INSTALL_MODE=release

# Install blkpy
RUN echo "INSTALL_MODE is set to: $INSTALL_MODE" && \
    if [ "$INSTALL_MODE" = "develop" ]; then \
        echo "Running python setup.py develop" && \
        python setup.py develop; \
    else \
        echo "Running python setup.py install" && \
        python setup.py install; \
    fi
