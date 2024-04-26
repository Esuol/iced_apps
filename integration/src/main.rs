mod controls;
mod scene;

use controls::Controls;
use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};
use iced_winit::core::{mouse, renderer, window, Color, Font, Pixels, Size, Theme};
use iced_winit::runtime::{Debug, Program};
use iced_winit::{conversion, futures, winit, Clipboard};
use scene::Scene;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    println!("Hello, world!");
}
