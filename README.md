# kp-rs

Утиліта командного рядка написана на Rust для автоматизації процесу завантаження та збірки програм з вихідного коду.

## Можливості

- Автоматичне завантаження архівів з вихідним кодом
- Розпакування архівів
- Підтримка систем збірки:
    - Meson
    - Configure (Autotools)
- Автоматична збірка та встановлення

## Встановлення

Для роботи програми потрібно мати встановлений Rust та cargo. 

```
git clone https://github.com/Koziev/kp-rs.git
cd kp-rs
cargo build --release
cargo install --path .
```

## Використання

Базовий синтаксис:
```
kp-rs --tool <СИСТЕМА_ЗБІРКИ> --url <URL_АРХІВУ> [--options <ДОДАТКОВІ_ОПЦІЇ ДЛЯ СИСЬЕМИ ЗБІРКИ>]
```

### Параметри

- `--tool, -t` (обов'язковий): Система збірки (`meson` або `configure`)
- `--url, -u` (обов'язковий): URL-адреса архіву з вихідним кодом
- `--options, -o` (необов'язковий): Додаткові опції для системи збірки

### Приклади

Збірка проєкту з використанням Meson:
```
kp-rs -t meson -u https://www.kernel.org/pub/linux/utils/kernel/kmod/kmod-34.tar.xz
```

Збірка проєкту з використанням Configure з додатковими опціями:
```
kp-rs --tool configure --url https://www.kernel.org/pub/linux/utils/kernel/kmod/kmod-34.tar.xz --options '--buildtype=release'
```




