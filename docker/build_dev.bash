#!/usr/bin/env bash

docker build -t base_rust_arduino:latest -f docker/Dockerfile.base .
docker build -t dev_rust_arduino:latest -f docker/Dockerfile.dev .

