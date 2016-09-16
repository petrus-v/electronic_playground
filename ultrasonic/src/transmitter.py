# Import required Python libraries
from datetime import datetime
import random
import RPi.GPIO as GPIO
import time

# Use BCM GPIO references
# instead of physical pin numbers
GPIO.setmode(GPIO.BCM)

# Define GPIO to use on Pi
GPIO_TRIGGER = 23

print("Ultrasonic Measurement")

# Set pins as output and input
GPIO.setup(GPIO_TRIGGER, GPIO.OUT)  # Trigger

# Set trigger to False (Low)
GPIO.output(GPIO_TRIGGER, False)

# Allow module to settle
time.sleep(0.5)

while True:
    # Send 10us pulse to trigger
    GPIO.output(GPIO_TRIGGER, True)
    time.sleep(0.00001)
    GPIO.output(GPIO_TRIGGER, False)
    time.sleep(0.00001)
    wait_time = random.random() * 10
    print("%s - Waiting %.1f secondes" % (
        datetime.now().strftime('%H:%M:%S'),
        wait_time
    ))
    time.sleep(wait_time)

# Reset GPIO settings
GPIO.cleanup()
