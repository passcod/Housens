# Components

- BME280 sensor
  - Temperature, Humidity, Pressure
  - 3.6 uA @ 1Hz typical
  - 1.8V power
- OPT3005 sensor
  - Ambient light, with IR filter
  - 1.8 uA typical
  - 1.8V power
- STM32L0
  - 1.8V power
  - 50 uA per MHz
  - Rust support
- CR2032
  - 2.7V flat
  - 3V nominal
  - 3.4V max
  - 210 mAh typical
- MCP1603
  - Voltage switching regulator
  - 2.7 - 4.5V input
  - 1.8V output
  - 45 uA quiescent, 0.1 uA shutdown
  - 500 mA max output

## Power usage

- load: 3.6 uA + 1.8 uA + 50 uA * 40 + 45 uA = ~2mA
- so ~100 hour battery life @ 1Hz update constant power
