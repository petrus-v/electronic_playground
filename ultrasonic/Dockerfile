FROM rpi-alpine-python:3.5

RUN mkdir -p /usr/src/ultrasonic
WORKDIR /usr/src/ultrasonic

COPY requirements.txt /usr/src/ultrasonic
RUN apk add --no-cache g++ && \
    pip install --no-cache-dir -r requirements.txt

COPY ultrasonic.py /usr/src/ultrasonic/ultrasonic.py
CMD ["python3", "ultrasonic.py"]
