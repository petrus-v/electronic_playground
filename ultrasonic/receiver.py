# Import required Python libraries
from datetime import datetime
import time
import RPi.GPIO as GPIO

# Use BCM GPIO references
# instead of physical pin numbers
GPIO.setmode(GPIO.BCM)

# Define GPIO to use on Pi
GPIO_ECHO = 24

print("Ultrasonic Measurement")

# Set pins as output and input
GPIO.setup(GPIO_ECHO, GPIO.IN)      # Echo

# Allow module to settle
time.sleep(0.5)

# Send 10us pulse to trigger
while True:
    start = time.time()
    while GPIO.input(GPIO_ECHO) == 0:
        start = time.time()

    while GPIO.input(GPIO_ECHO) == 1:
        stop = time.time()

    # Calculate pulse length
    elapsed = stop-start

    print("%s - elapsed time: %.1f" % (
        datetime.now().strftime('%H:%M:%S'),
        elapsed,
    ))

# Reset GPIO settings
GPIO.cleanup()
