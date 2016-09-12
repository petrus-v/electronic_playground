# Alpine python3 support for RPI.

This [Docker file](https://github.com/docker-library/python/tree/master/3.5/alpine)
came from docker-library team with the main change that is based on [hypriot](
https://github.com/hypriot/rpi-alpine-scratch) works.


build:

```
  cd ~/electronic_playground/docker-rpi/alpine/python
  docker build -t rpi-alpine-python:3.5 .
```
