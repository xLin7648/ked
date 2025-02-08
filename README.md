# 这是什么

一个一个（X）winit + wgpu的跨平台框架.

## 支持的平台

* Windows;
* Android(测试);

## 构建说明

### 设置项目

clone这个包

解压后将其添加到 Cargo.toml 中:
```toml

[dependencies]
ked ={ path = "ked根目录" }
```

在 `src/main.rs` 中编写:
```rust
use ked::*;

#[derive(Default)]
pub struct MyGame;

impl GameLoop for MyGame {
    fn start(&mut self, _c: &mut EngineContext) {
    }

    fn update(&mut self, _c: &mut EngineContext) {
        _c.timer.println_time_data();
    }
}

#[ked_main(LevelFilter::Off)]
fn main(event_loop: EventLoop<()>) {
    let mut window_config = WindowConfig::default();
    window_config.clear_color = WHITE;

    let game = MyGame::default();

    App::new(game, window_config).run(event_loop);
}
```

然后cargo run:
```bash
cargo run
```
