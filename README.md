# egui-user-defined-font
This example show how to load userdefined font for egui 0.29.1 . AlibabaPuHuiTi-3-55-Regular.ttf download from https://fonts.alibabagroup.com/#/home . This is a font released globally for free commercial use by China's Alibaba, covering 178 languages.



# Load from fonts path
Please place the .ttf or .otf font files in the 'fonts/' subdirectory of the program directory.

```bash
cargo run --release --features font_path
```

# embed in exec file
Please place the .ttf or .otf font files in the 'resource/' subdirectory of the src directory. By default, AlibabaPuHuiTi-3-55-Regular.ttf has placed into 'src/resource/' (download from https://fonts.alibabagroup.com/#/home) . This is a font released globally for free commercial use by China's Alibaba, covering 178 languages.
```bash
cargo run --release
```
