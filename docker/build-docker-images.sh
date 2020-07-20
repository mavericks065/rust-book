#!/usr/bin/env bash

docker build --no-cache -f ./Dockerfile-postgres-test -t rust-book-postgres:latest .
