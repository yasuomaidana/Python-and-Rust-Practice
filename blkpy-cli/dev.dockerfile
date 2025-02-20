FROM python:3.10-alpine

# Install lsblk
RUN apk add --no-cache util-linux

WORKDIR /usr/src/app

# Install dependencies
COPY requirements.txt .
RUN pip install -r requirements.txt