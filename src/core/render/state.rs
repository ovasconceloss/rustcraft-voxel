use std::sync::Arc;
use winit::window::Window;

pub struct State {
  queue: wgpu::Queue,
  window: Arc<Window>,
  device: wgpu::Device,
  surface: wgpu::Surface<'static>,
  config: wgpu::SurfaceConfiguration,
  render_pipeline: wgpu::RenderPipeline
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

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("Shader"),
      source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("Render Pipeline Layout"),
      bind_group_layouts: &[],
      push_constant_ranges: &[]
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("Render Pipeline"),
      layout: Some(&render_pipeline_layout),
      vertex: wgpu::VertexState {
        module: &shader,
        entry_point: Some("vs_main"),
        buffers: &[],
        compilation_options: wgpu::PipelineCompilationOptions::default()
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader,
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: config.format,
          blend: Some(wgpu::BlendState::REPLACE),
          write_mask: wgpu::ColorWrites::ALL
        })],
        compilation_options: wgpu::PipelineCompilationOptions::default()
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: Some(wgpu::Face::Back),
        polygon_mode: wgpu::PolygonMode::Fill,
        unclipped_depth: false,
        conservative: false
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState {
        count: 1,
        mask: !0,
        alpha_to_coverage_enabled: false
      },
      multiview: None,
      cache: None
    });

    Self { queue, window, device: device, surface, config, render_pipeline }
  }
}