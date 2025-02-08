use crate::*;

pub struct App {
    timer: Time,
    game: Box<dyn GameLoop>,

    wr: Option<WgpuRenderer>,
    window: Option<Arc<Window>>,
    window_config: Arc<Mutex<WindowConfig>>,
}

impl App {
    pub fn new(
        game: impl GameLoop + 'static, 
        window_config: WindowConfig
    ) -> Self {
        Self { 
            game: Box::new(game),
            window_config: Arc::new(Mutex::new(window_config)),

            timer: Time::new(),

            wr: None,
            window: None,
        }
    }

    pub fn run(
        &mut self, 
        event_loop: winit::event_loop::EventLoop<()>, 
        control_flow: ControlFlow
    ) {
        event_loop.set_control_flow(control_flow);
        let _ = event_loop.run_app(self);
    }

    fn init_window(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_config = &self.window_config.lock();
    
        let resolution = Some(match window_config.resolution {
            ResolutionConfig::Physical(w, h) => Size::Physical(PhysicalSize::new(w, h)),
            ResolutionConfig::Logical(w, h) => Size::Logical(LogicalSize::new(w as f64, h as f64)),
        });
        let min_resolution = Some( match window_config.min_resolution {
            ResolutionConfig::Physical(w, h) => Size::Physical(PhysicalSize::new(w, h)),
            ResolutionConfig::Logical(w, h) => Size::Logical(LogicalSize::new(w as f64, h as f64)),
        });
    
        let fullscreen = if window_config.fullscreen {
            Some(Fullscreen::Borderless(None))
        } else {
            None
        };
    
        let mut window_attributes = WindowAttributes::default();
    
        window_attributes.title          = window_config.title_name.clone();
        window_attributes.inner_size     = resolution;
        window_attributes.min_inner_size = min_resolution;
        window_attributes.fullscreen     = fullscreen;
    
        self.window = Some(Arc::new(event_loop.create_window(window_attributes).unwrap()));
    }

    fn init_wr(&mut self) {
        self.wr = Some(
            WgpuRenderer::new(
                self.window.clone().unwrap(),
                Arc::clone(&self.window_config),
            )
            .block_on()
        );
    }

    fn get_components(&mut self) -> (
        &mut Box<dyn GameLoop>,
        &Time, &mut WgpuRenderer, 
    ) {
        (
            &mut self.game,
            &self.timer,
            self.wr.as_mut().unwrap()
        )
    }

    fn renderer_update(&mut self) {
        if let Some(c) = &mut self.wr {
            /* c.update();
            c.draw();
            c.end_frame();

            clear_shader_uniform_table(); */
        }
    }
}

impl ApplicationHandler for App {
        // 当应用程序从挂起状态恢复时调用此方法
        fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            if let Some(window) = &self.window {
                if let Some(wr) = &mut self.wr {
                    wr.context.resume(window.clone());
    
                    info!("Resumed");
                } else {
                    self.init_wr();
                }
            } else {
                self.init_window(event_loop);
                self.init_wr();

                // 在这里Start
                {
                    let (game, timer, renderer) = self.get_components();
                    let mut c = EngineContext { renderer, timer };
        
                    game.start(&mut c);
                }
    
                info!("InitWindow");
            }
        }
    
        // 在事件循环即将等待输入事件时调用
        fn about_to_wait(&mut self, _: &ActiveEventLoop) {
            if self.wr.is_some() {
                self.timer.update();

                let (game, timer, renderer) = self.get_components();
                let mut c = EngineContext { renderer, timer };
    
                game.update(&mut c);
        
                self.renderer_update();
            }
        }
    
        // 处理窗口相关的事件
        fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _: winit::window::WindowId,
            event: winit::event::WindowEvent,
        ) {
            match event {
                WindowEvent::Resized(new_size) => {
                    if let Some(wr) = &mut self.wr {
                        wr.resize(new_size);
    
                        self.about_to_wait(event_loop);
                    }
                },
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                },
                _ => (),
            }
        }
    
        // region: 看起来没什么用的内容
    
        // 当应用程序被挂起时调用
        fn suspended(&mut self, _: &ActiveEventLoop) {
            if let Some(wr) = self.wr.as_mut() {
                wr.context.surface.take();
            }
            
            info!("Suspended");
        }
    
        // 在应用程序准备退出时调用
        fn exiting(&mut self, _: &ActiveEventLoop) {
            info!("Exiting");
        }
    
        // endregion: 看起来没什么用的内容
}

pub struct EngineContext<'a> {
    pub timer: &'a Time,
    pub renderer: &'a mut WgpuRenderer,
}