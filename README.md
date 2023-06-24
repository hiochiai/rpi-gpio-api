# RPi GPIO API

RPi GPIO API is a REST API server for controlling Raspberry Pi GPIO.

## Getting started

### Install

Download deb package and install to your Raspberry Pi.

```
$ curl -sSL -f -o rpi-gpio-api.deb 'https://github.com/hiochiai/rpi-gpio-api/releases/download/v0.1.0/rpi-gpio-api_0.1.0_armhf.deb'
$ sudo dpkg -i rpi-gpio-api.deb
```

API Server starts with port 8008.  
If you want to change the port, edit `/lib/systemd/system/rpi-gpio-api.service` and run `sudo systemctl daemon-reload && sudo systemctl restart rpi-gpio-api.service`.

### Use API

[API Document](https://hiochiai.github.io/rpi-gpio-api/)

#### Get GPIO 23 (Physical/Board pin 16)

Request:
```
$ curl http://raspberrypi.local:8008/api/gpio/23
```

Response Example:
```
{"function":"input","level":"low"}
```

#### Set GPIO 23 (Physical/Board pin 16)

Request:
```
$  curl -X PUT -H "Content-Type: application/json" -d '{"function":"output","level":"high"}' http://raspberrypi.local:8008/api/gpio/23
```

Response Example:
```
{"function":"output","level":"high"}
```

## Build

```
$ sudo apt-get update && sudo apt-get install gcc-arm-linux-gnueabihf -y
$ rustup target install armv7-unknown-linux-gnueabihf
$ cargo build --release
```

## Package

`make` and `docker` are required.

```
$ make package
```

*This command will create docker image `rpi-gpio-api-builder:local` on running machine.*
