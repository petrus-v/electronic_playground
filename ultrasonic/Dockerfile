FROM rpi-alpine-python:3.5

RUN mkdir -p /usr/src/ultrasonic
WORKDIR /usr/src/ultrasonic

COPY ./src /usr/src/ultrasonic
RUN apk add --no-cache g++ && \
    pip install --no-cache-dir -r requirements.txt

CMD ["python3", "ultrasonic.py"]
