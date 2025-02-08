mod app;
mod time;
mod color;
mod config;
mod device;
mod graphics;
mod gameloop;

pub use app::*;
pub use time::*;
pub use color::*;
pub use config::*;
pub use gameloop::*;

pub use log;
pub use log::*;

#[cfg(target_os = "android")]
pub use android_logger;

#[cfg(not(target_os = "android"))]
pub use env_logger;

pub use glam::*;
pub use winit::event_loop::*;
pub use ked_main_macro::ked_main;
pub use wgpu::{PresentMode, PowerPreference};

use pollster::*;
use parking_lot::*;
use once_cell::sync::*;

use device::*;
use graphics::*;
use std::sync::Arc;

// Winit 相关的导入
use winit::{
    dpi::*,
    event::*,
    event_loop::*,
    window::*
};

use winit::application::ApplicationHandler;

// WGPU 相关的导入
use wgpu::{
    util::{self, DeviceExt}, Adapter, Backends, BindGroup, 
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, 
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, 
    Buffer, BufferBindingType, BufferUsages, Device, DeviceDescriptor, 
    Features, Instance, InstanceDescriptor, Limits, PipelineCompilationOptions, 
    Queue, ShaderStages, Surface, SurfaceConfiguration
};