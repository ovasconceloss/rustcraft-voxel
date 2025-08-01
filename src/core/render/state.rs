use std::sync::Arc;
use winit::{keyboard::KeyCode, window::Window};

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

  pub fn handle_key(&self, event_loop: &winit::event_loop::ActiveEventLoop, code: KeyCode, is_pressed: bool) {
    match (code, is_pressed) {
      (KeyCode::Escape, true) => event_loop.exit(),
      (KeyCode::KeyW, true) => println!("W"),
      (KeyCode::KeyA, true) => println!("A"),
      (KeyCode::KeyS, true) => println!("S"),
      (KeyCode::KeyD, true) => println!("D"),
      (KeyCode::KeyF, true) => println!("F"),
      _ => {}
    }
  }
  
  pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
    self.window.request_redraw();

    let output = self.surface.get_current_texture()?;
    let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
      label: Some("Render Encoder"),
    });

    {
      let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
          view: &view,
          resolve_target:None,
          depth_slice: None,
          ops: wgpu::Operations {
            load: wgpu::LoadOp::Clear(wgpu::Color { r: 0.2, g: 0.2, b: 0.3, a: 1.0 }),
            store: wgpu::StoreOp::Store
          },
        })],
        depth_stencil_attachment: None,
        occlusion_query_set: None,
        timestamp_writes: None
      });
    }

    self.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
  }
}