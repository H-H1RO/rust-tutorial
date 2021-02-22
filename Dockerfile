FROM rust:1.50-alpine

WORKDIR /usr/src/app

RUN apt-get update \
    && apt-get -y install vim
