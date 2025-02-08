use crate::*;

pub struct GraphicsContext {
    pub surface: Option<Arc<Surface<'static>>>,
    pub instance: Arc<Instance>,
    pub adapter: Arc<Adapter>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub config: Arc<Mutex<SurfaceConfiguration>>
}

pub struct WgpuRenderer {
    pub context: GraphicsContext,
    pub size: PhysicalSize<u32>,
}

impl WgpuRenderer {
    pub async fn new(
        window: Arc<Window>,
        window_config: Arc<Mutex<WindowConfig>>
    ) -> Self { 
        let size = window.inner_size();
        let context = create_graphics_context(window, window_config).await;
        
        Self {
            context,

            size
        }
    }

    pub fn resize(&mut self, mut new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            new_size.width = new_size.width.max(1);
            new_size.height = new_size.height.max(1);
            self.size = new_size;

            /* if let Some(main_camera) = &self.main_camera {
                main_camera.lock().resize(new_size);
            } */

            if let Some(surface) = &self.context.surface.as_mut() {
                let mut config = 
                self.context.config.lock();

                config.width = new_size.width;
                config.height = new_size.height;

                surface.configure(&self.context.device, &config);
            }

            self.update_resources();
        }
    }

    fn update_resources(&mut self) {
        /* self.msaa_texture = create_multisampled_framebuffer(
            &self.context.device, 
            &self.context.config.borrow(), 
            window_config().sample_count.clone().into()
        ); */
    }
}

impl GraphicsContext {
    pub fn resume(&mut self, window: Arc<Window>) {
        // Window size is only actually valid after we enter the event loop.
        let window_size = window.inner_size();
        let width = window_size.width.max(1);
        let height = window_size.height.max(1);

        info!("Surface resume {window_size:?}");

        let surface = self.instance.create_surface(window).unwrap();

        let mut config = self.config.lock();

        config.width = width;
        config.height = height;

        surface.configure(&self.device, &config);

        self.surface = Some(Arc::new(surface));
    }
}