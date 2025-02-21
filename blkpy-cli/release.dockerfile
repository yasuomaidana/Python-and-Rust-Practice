FROM python:3.10-alpine
RUN apk add --no-cache util-linux
COPY dist/blkpy_demo-0.1.0-py3-none-any.whl .
RUN pip install blkpy_demo-0.1.0-py3-none-any.whl
ENTRYPOINT ["sh"]