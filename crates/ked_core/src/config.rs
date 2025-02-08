/*
 * source: https://github.com/darthdeus/comfy
 */

use crate::*;

#[derive(Copy, Clone, Debug)]
pub enum ResolutionConfig {
    Physical(u32, u32),
    Logical(u32, u32),
}

impl ResolutionConfig {
    pub fn width(&self) -> u32 {
        match self {
            Self::Physical(w, _) => *w,
            Self::Logical(w, _) => *w,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Physical(_, h) => *h,
            Self::Logical(_, h) => *h,
        }
    }

    pub fn ensure_non_zero(&mut self) -> ResolutionConfig {
        const MIN_WINDOW_SIZE: u32 = 1;
        match self {
            ResolutionConfig::Physical(w, h) |
            ResolutionConfig::Logical(w, h)
                if *w == 0 || *h == 0 =>
            {
                *w = MIN_WINDOW_SIZE;
                *h = MIN_WINDOW_SIZE;
            }
            _ => (),
        }

        *self
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum Msaa {
    Off = 1,
    Sample2 = 2,
    #[default]
    Sample4 = 4,
    Sample8 = 8,
}

// 实现 From Trait，使其返回对应的 u32 值
impl From<Msaa> for u32 {
    fn from(msaa: Msaa) -> Self {
        msaa as u32
    }
}

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title_name: String,
    pub version: &'static str,
    pub fullscreen: bool,

    pub resolution: ResolutionConfig,
    pub min_resolution: ResolutionConfig,
    
    pub sample_count: Msaa,
    pub vsync_mode: PresentMode,
    pub power_preference: PowerPreference,

    pub clear_color: Color
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self { 
            title_name: "New Game".to_owned(),
            version: "New Version",
            fullscreen: false,

            resolution: ResolutionConfig::Physical(1280, 720), 
            min_resolution: ResolutionConfig::Physical(100, 100), 

            sample_count: Msaa::default(),
            vsync_mode: PresentMode::default(),
            power_preference: PowerPreference::default(),

            clear_color: BLUE
        }
    }
}
/* 
pub fn clear_background(color: Color) {
    let mut last_config = WINDOW_CONFIG
        .get()
        .expect("window_config() must be called after comfy main runs")
        .borrow_mut();

    last_config.clear_color = color;
} */
