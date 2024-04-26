mod controls;
mod scene;

use controls::Controls;
use iced::clipboard;
use iced::keyboard::Modifiers;
use iced_wgpu::graphics::futures::backend::default;
use iced_wgpu::graphics::Viewport;
use iced_wgpu::wgpu::naga::back;
use iced_wgpu::{wgpu, Engine, Renderer};
use iced_winit::core::{mouse, renderer, window, Color, Font, Pixels, Size, Theme};
use iced_winit::runtime::{Debug, Program};
use iced_winit::{conversion, futures, winit, Clipboard};
use scene::Scene;

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
};

use std::sync::Arc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlCanvasElement;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowBuilderExtWebSys;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_arch = "wasm32")]
    let canvas_element = {
        console_log::init().expect("Initialization logger");

        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.get_element_by_id("iced_canvas"))
            .and_then(|canvas| canvas.dyn_into::<HtmlCanvasElement>().ok())
            .expect("Get canvas element")
    };

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;

    #[cfg(target_arch = "wasm32")]
    let window = winit::window::WindowBuilder::new()
        .with_canvas(Some(canvas_element))
        .build(&event_loop)?;

    #[cfg(not(target_arch = "wasm32"))]
    let window = wint::window::Window::new(&event_loop)?;

    let windwo = Arc::new(window);

    let physical_size = window.inner_size();
    let mut viewport = Viewport::with_physical_size(
        Size::new(physical_size.width, physical_size.height),
        window.scale_factor(),
    );
    let mut cursor_position = None;
    let mut modifiers = ModifiersState::default();
    let mut clipboard = Clipboard::connect(&window)?;

    // Intialize the WGPU
    #[cfg(target_arch = "wasm32")]
    let default_backend = wgpu::Backends::GL;
    #[cfg(not(target_arch = "wasm32"))]
    let default_backend = wgpu::Backends::PRIMARY;

    let backend = wgpu::util::backend_bits_from_env().unwrap_or(default_backend);

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: backend,
        ..Default::default()
    });

    let surface = instance.create_surface(window.clone())?;

    let (format, adapter, device, queue) = futures::futures::executor::block_on(async {
        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
            .await
            .expect("Create adapter");

        let adapter_features = adapter.features();

        #[cfg(target_arch = "wasm32")]
        let needed_limits =
            wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits());

        #[cfg(not(target_arch = "wasm32"))]
        let needed_limits = wgpu::Limits::default();

        let capabilities = surface.get_capabilities(&adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: adapter_features & wgpu::Features::default(),
                    required_limits: needed_limits,
                },
                Node,
            )
            .await
            .expect("Request device");

        (
            capabilities
                .formats
                .iter()
                .copied()
                .find(wgpu::TextureFormat::is_srgb)
                .or_else(|| capabilities.formats.first().copied())
                .expect("Get preferred format"),
            adapter,
            device,
            queue,
        )
    });
}
