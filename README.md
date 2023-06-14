# OnyxBay14.Drivers

Здесь расположен код драйверов различных устройств в OnyxBay14.

## Требования

- [Rust](https://www.rust-lang.org/tools/install) - чем новее - тем лучше.

## Структура

```sh
.
├─ apm/ -- драйвер Advanced Power Management, получение заряда батареи, выключение/перезагрузка.
├─ bsod/ -- библиотека для красивого вывода паник.
├─ clint/ -- драйвер Core Local Interrupter, программные и таймерные прерывания.
├─ gpu/ -- драйвер для GPU.
├─ health_analyzer/ -- драйвер для устройства анализатора здоровья.
├─ heap/ -- готовый глобальный аллокатор.
├─ hid/ -- драйверы Human Interface Device - клавиатура и мышь.
├─ mmio/ -- драйвер для MMIO устройств.
├─ net_hub/ -- драйвер сетевого концентратора.
├─ pci/ -- драйвер PCI шины.
├─ plic/ -- драйвер Platform Level Interrupt Controller, управление внешними прерываниями.
├─ rtc/ -- драйвер Real Time Clock, получение реального времени.
├─ screen/ -- драйвер монитора.
├─ serial_terminal/ -- драйвер последовательного устройства.
├─ sgl/ -- Simple Graphics Library, библиотека для работы с графикой.
├─ stack_string/ -- небольшие строки на стэке.
└─ tts/ -- драйвер для TTS устройств.
```

## Примечания

Перед использованием любой графики требуется перключить Float-point extension State регистр!

```rs
riscv::register::mstatus::set_fs(FS::Initial);
```
