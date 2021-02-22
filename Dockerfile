FROM rust:1.50-alpine

WORKDIR /usr/src/app

RUN apk update \
    && apk add vim
