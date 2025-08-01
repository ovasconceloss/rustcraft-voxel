use std::sync::Arc;
use winit::window::Window;

pub struct State {
  queue: wgpu::Queue,
  window: Arc<Window>,
  device: wgpu::Device,
  surface: wgpu::Surface<'static>,
  config: wgpu::SurfaceConfiguration,
}

impl State {
  pub async fn new(window: Arc<Window>) -> Self {
    let size = window.inner_size();
    
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
      backends: wgpu::Backends::all(),
      ..Default::default()
    });

    let surface = instance.create_surface(window.clone()).unwrap();
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
      power_preference: wgpu::PowerPreference::HighPerformance,
      force_fallback_adapter: false,
      compatible_surface: Some(&surface)
    })
    .await
    .unwrap();

    let (device, queue) = adapter.request_device(
      &wgpu::DeviceDescriptor {
        label:None,
        trace: wgpu::Trace::Off,
        memory_hints: Default::default(),
        required_limits: wgpu::Limits::defaults(),
        required_features: wgpu::Features::default(),
      }
    )
    .await
    .unwrap();

    let surface_caps = surface.get_capabilities(&adapter);
    let surface_format = surface_caps.formats
      .iter().find(|f| f.is_srgb())
      .copied().unwrap_or(surface_caps.formats[0]);

    let config = wgpu::SurfaceConfiguration {
      usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
      format: surface_format,
      width: size.width,
      height: size.height,
      present_mode: surface_caps.present_modes[0],
      alpha_mode: surface_caps.alpha_modes[0],
      view_formats: vec![],
      desired_maximum_frame_latency: 2
    };

    surface.configure(&device, &config);

    Self { queue, window, device: device, surface, config }
  }
}