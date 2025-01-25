# Components

- BME280 sensor
  - Temperature, Humidity, Pressure
  - 3.6 uA @ 1Hz typical
  - 1.8V power
- OPT3005 sensor
  - Ambient light, with IR filter
  - 1.8 uA typical
  - 1.8V power
- STM32WB
  - STM32WB55CG   (LCD,   1MB flash / 256kB ram), or:
    - STM32WB55CE (LCD, 512kB flash / 256kB ram)
    - STM32WB55CC (LCD, 256kB flash / 128kB ram)
    - STM32WB35CE (---, 512kB flash /  96kB ram)
    - STM32WB35CC (---, 256kB flash /  96kB ram)
  - ARM Cortex-M4
  - BLE5 and Thread
  - 1.8V power
  - 90 uA per MHz, up to 64MHz
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

- load: 3.6 uA + 1.8 uA + 90 uA * 64 + 45 uA = ~6mA
- so ~36 hour battery life @ 1Hz update constant power
