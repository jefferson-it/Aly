# IoT and Embedded Support in Aly

Aly supports IoT and embedded platforms through feature-gated modules for ESP32, Arduino, Raspberry Pi, and low-level protocols (GPIO, I2C, SPI, UART).

> **Note**: These modules require enabling the corresponding Cargo features at build time.

---

## 1. ESP32

```aly
import esp32

esp32.init()
esp32.set_pin_mode(2, "output")
esp32.digital_write(2, "high")
```

---

## 2. Arduino

```aly
import arduino

arduino.setup()
arduino.pin_mode(13, "OUTPUT")
arduino.digital_write(13, "HIGH")
```

---

## 3. Raspberry Pi

```aly
import raspberry_pi

rpi.init()
rpi.setup_gpio(17, "out")
rpi.output(17, true)
```

---

## 4. GPIO (General Purpose Input/Output)

```aly
import gpio

gpio.set_mode(17, "output")
gpio.write(17, true)
let value = gpio.read(18)
```

---

## 5. I2C Protocol

```aly
import i2c

let bus = i2c.open(1)       # Open I2C bus 1
i2c.write(bus, 0x48, [0x00, 0xFF])
let data = i2c.read(bus, 0x48, 2)
i2c.close(bus)
```

---

## 6. SPI Protocol

```aly
import spi

spi.begin(0, 0)              # SPI bus 0, chip select 0
spi.set_speed(1000000)        # 1 MHz
let result = spi.transfer([0x01, 0x02, 0x03])
spi.end()
```

---

## 7. UART / Serial

```aly
import serial

let port = serial.open("/dev/ttyUSB0", 115200)
serial.write(port, "AT\r\n")
let response = serial.read(port, 100)
serial.close(port)
```