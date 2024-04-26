use iced_wgpu::wgpu;
use iced_winit::core::Color;

pub struct Scene {
    pipeline: wgpu::RenderPipeline,
}

impl Scene {
    pub fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Scene {
        let pipeline = build_pipeline(device, texture_format);

        Scene { pipeline }
    }
}

fn build_pipeline(
  device: &wgpu::Device,
  texture_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
  let (vs_module, fs_module) = (
    device.create_shader_module(wgpu::include_wgsl!("shader/vert.wgsl")),
    device.create_shader_module(wgpu::include_wgsl!("shader/frag.wgsl")),
  );
}
