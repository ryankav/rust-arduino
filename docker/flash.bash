#!/usr/bin/env bash

while [[ $# -gt 0 ]]; do
  case $1 in
    -d|--device)
      # Set the path to the device so we can flash onto it
      DEVICE_PATH="$2"
      shift # past argument
      shift # past vlaue
      ;;
    -*|--*)
      echo "Unknown option $1"
      exit 1;
      ;;
  esac
done

if [ -z "$DEVICE_PATH" ]; then
  echo "Need to input path for the arduino device";
  exit 1;
fi

if ! [ -e "$DEVICE_PATH" ]; then
  echo "Path not found";
  exit 1;
fi


docker build -t base_rust_arduino:latest -f docker/Dockerfile.base .
docker build --build-arg DEVICE_PATH="${DEVICE_PATH}" -t flash_rust_arduino:latest -f docker/Dockerfile.flash .
docker run -d --device=${DEVICE_PATH}:${DEVICE_PATH} flash_rust_arduino

