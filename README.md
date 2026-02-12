<div>
    <div align="center">
        <h1>Проектная работа №4 курса "Rust для действующих разработчиков"</h1>
    </div>
</div>


# Структура проекта

- [CLI утилита для работы с PNG](image_processor/README.md)
- [Плагин для размытия изображения](blur_plugin/README.md)
- [Плагин для зеркального разворота изображения](blur_plugin/README.md)

## Настройка локальног оокружения

1. Установка RUST на Linux/MacOS
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
```bash
# Проверить готовность к работе
rustc --version && \
cargo --version
```

2. Клонировать проект
```bash
git clone https://github.com/daniilburavlev/yandex-module-4.git
```

3. Сборка проекта
```bash
cargo build --release
```

4. Тестирование
```bash
cargo test
```
