<div>
    <div align="center">
        <h1>Blur Plugin</h1>
    </div>
</div>


# Описание
Плагин для размытия, принимает два параметра через текстовый файл 

- `radius` - радиус размытия
- `iterations` - количество итераций

## Пример [файла](params.json) с параметрами

## Пример использования
Скомпилировать библиотеку

```bash
cargo build
```

Запустить CLI

```bash
cargo run \
  --bin image_processor \
  -- --input image.png \
  --output output.png \
  --params blur_plugin/params.txt -- \
  plugin libblur_plugin.dylib
```
```
```
