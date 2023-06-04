# OnyxBay14.Drivers

Здесь расположен код драйверов различных устройств в OnyxBay14.

## Требования

- [Rust](https://www.rust-lang.org/tools/install) - чем новее - тем лучше.

## Структура

```sh
.
├─ apm/ -- драйвер Advanced Power Management, получение заряда батареи, выключение/перезагрузка.
├─ gpu/ -- драйвер для GPU.
├─ heap/ -- готовый глобальный аллокатор.
├─ hid/ -- драйверы Human Interface Device - клавиатура и мышь.
├─ mmio/ -- драйвер для MMIO устройств.
├─ pci/ -- драйвер PCI шины.
├─ plic/ -- драйвер Platform Level Interrupt Controller, управление внешними прерываниями.
├─ rtc/ -- драйвер Real Time Clock, получение реального времени.
├─ screen/ -- драйвер монитора.
├─ serial_terminal/ -- драйвер последовательного устройства.
└─ tts/ -- драйвер для TTS устройств.
```
