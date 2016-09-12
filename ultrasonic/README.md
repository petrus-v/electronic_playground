# Ultrasonic

A simple project with ultrasonic sensor (HC-SR04) as coverded by a lot of
tutorials:

http://www.manuel-esteban.com/lire-un-capteur-ultrason-hc-sr04-avec-un-raspberry-pi/
http://www.framboise314.fr/mesure-de-distance-par-ultrasons-avec-le-raspberry-pi/
http://raspberry-pi.developpez.com/cours-tutoriels/capteur/mag-pi-utiliser-port-gpio/partie-3-telemetre-ultrason/
https://espaceraspberryfrancais.shost.ca/Composants/Mesure-de-distance-avec-HC-SR04-Raspberry-Francais/
https://www.modmypi.com/blog/hc-sr04-ultrasonic-range-sensor-on-the-raspberry-pi
...

build:

>  docker build -t 00-ultrasonic .


run:

>  docker run --rm -it -v /home/pirate/electronic_sandbox/ultrasonic/:/usr/src/ultrasonic/ \
>             --cap-add SYS_RAWIO --device /dev/mem 00-ultrasonic 
