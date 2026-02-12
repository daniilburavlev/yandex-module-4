<div>
    <div align="center">
        <h1>Mirror Plugin</h1>
    </div>
</div>


# Описание
Плагин для зеркально отображения 

- `V` - отобразить по вертикали 
- `H` - отобразить по горизонтали 

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
  --params mirror_plugin/params.txt -- \
  plugin libmirror_plugin.dylib
```
```
```
