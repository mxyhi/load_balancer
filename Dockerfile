FROM alpine:latest AS base

WORKDIR /app

RUN apk add --no-cache bash

COPY ./bin /app/bin

COPY ./conf.yaml /app/conf.yaml

COPY ./start.sh /app/start.sh

ENV PATH="/app/bin:${PATH}"

ENTRYPOINT [ "/bin/bash", "-c" ]

CMD ["/app/start.sh"]
