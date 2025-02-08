use crate::*;

use std::time::{Duration, Instant};

pub struct Time {
    start_time: Instant,
    current_time: Duration,
    delta_time: Duration,
    fps: f64,
    frame_count: usize,
    last_time: Instant,
}

impl Time {
    /// 创建一个新的 Time 实例
    pub fn new() -> Self {
        let start_time = Instant::now();
        Self {
            start_time,
            current_time: Duration::new(0, 0),
            delta_time: Duration::new(0, 0),
            fps: 0.0,
            frame_count: 0,
            last_time: start_time,
        }
    }

    /// 更新时间数据并计算 delta_time 和 fps
    pub fn update(&mut self) {
        let now = Instant::now();
        self.current_time = now.duration_since(self.start_time);
        
        // 计算 delta_time
        self.delta_time = now.duration_since(self.last_time);
        
        // 计算 fps
        if self.delta_time.as_secs_f64() > 0.0 { // 防止除以零
            self.fps = 1.0 / self.delta_time.as_secs_f64();
        }

        // 更新时间戳
        self.last_time = now;
    }

    /// 获取当前时间
    pub fn get_time(&self) -> f32 {
        self.current_time.as_secs_f32()
    }

    /// 获取 delta_time
    pub fn get_delta_time(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    /// 获取每秒帧数
    pub fn get_fps(&self) -> u32 {
        self.fps.round() as u32
    }

    pub fn println_time_data(&self) {
        println!(
            "time: {:.6}, deltaTime: {:.6}, fps: {}",
            self.current_time.as_secs_f32(),
            self.delta_time.as_secs_f32(),
            self.fps.round() as u32
        );
    }
}