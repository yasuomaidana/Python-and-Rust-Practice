FROM python:3.10-alpine

# Install lsblk
RUN apk add --no-cache util-linux

# Install dependencies
COPY requirements.txt .
COPY blkpy/ blkpy/
COPY lsblk_selector.py .
COPY setup.py .
RUN pip install -r requirements.txt
RUN if [ "$INSTALL_MODE" = "develop" ]; then python setup.py develop; else python setup.py install; fi