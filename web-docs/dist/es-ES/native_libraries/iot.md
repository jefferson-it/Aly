# Soporte IoT y Embebido en Aly

Aly admite plataformas IoT y embebidas a través de módulos con características específicas para ESP32, Arduino, Raspberry Pi y protocolos de bajo nivel (GPIO, I2C, SPI, UART).

> **Nota**: Estos módulos requieren habilitar las características de Cargo correspondientes en el momento de la compilación.

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

## 5. Protocolo I2C

```aly
import i2c

let bus = i2c.open(1)       # Abrir bus I2C 1
i2c.write(bus, 0x48, [0x00, 0xFF])
let data = i2c.read(bus, 0x48, 2)
i2c.close(bus)
```

---

## 6. Protocolo SPI

```aly
import spi

spi.begin(0, 0)              # Bus SPI 0, selección de chip 0
spi.set_speed(1000000)        # 1 MHz
let result = spi.transfer([0x01, 0x02, 0x03])
spi.end()
```

---

## 7. UART / Serie

```aly
import serial

let port = serial.open("/dev/ttyUSB0", 115200)
serial.write(port, "AT\r\n")
let response = serial.read(port, 100)
serial.close(port)
```