use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

struct State {
    window: Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        // Create a wgpu instance
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        // Find a GPU
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;

        let size = window.inner_size();

        // The surface is like where you draw things
        let surface = instance.create_surface(window.clone())?;
        // No idea on these two
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("default.wgsl").into()),
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        // This is so many nesting
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        let state = Self {
            window,
            device,
            queue,
            surface,
            surface_format,
            size,
            render_pipeline,
        };

        state.configure_surface();

        Ok(state)
    }

    fn get_window(&self) -> &Window {
        &self.window
    }

    fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        self.configure_surface();
    }

    fn render_with_context(&mut self, draw_handle: &RendiumDrawHandle, color: types::Color) {
        if draw_handle.vertices.is_empty() || draw_handle.indices.is_empty() {
            return;
        }

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&draw_handle.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&draw_handle.indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        let surface_texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(e) => {
                eprintln!("Failed to get surface texture: {:?}", e);
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.surface_format.add_srgb_suffix()),
                ..Default::default()
            });

        let mut encoder = self.device.create_command_encoder(&Default::default());

        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(color.into()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        renderpass.set_pipeline(&self.render_pipeline);
        renderpass.set_vertex_buffer(0, vertex_buffer.slice(..));
        renderpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        renderpass.draw_indexed(0..draw_handle.indices.len() as u32, 0, 0..1);

        drop(renderpass);
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        surface_texture.present();
    }
}

pub struct RendiumInstance {
    state: Option<State>,
    size: winit::dpi::PhysicalSize<u32>,
    title: String,
    callback: Box<dyn FnMut(&mut Self)>,
}

impl RendiumInstance {
    pub fn new(size: PhysicalSize<u32>, title: String, f: Box<dyn FnMut(&mut Self) -> ()>) -> Self {
        Self {
            size,
            title,
            state: None,
            callback: f,
        }
    }

    pub fn draw<F: FnOnce(&mut RendiumDrawHandle)>(&mut self, color: types::Color, f: F) {
        let mut draw_handle = RendiumDrawHandle::new(self.size);

        f(&mut draw_handle);
        if let Some(state) = &mut self.state {
            state.render_with_context(&draw_handle, color);
        }
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        (self.size.width, self.size.height)
    }
}

impl ApplicationHandler for RendiumInstance {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title(self.title.clone())
                        .with_inner_size(self.size),
                )
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone())).unwrap();
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed, exiting...");

                self.state = None;

                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // Since this is Rust, I have to jump through some hoops to make this work
                // (self.callback)(self);

                // Move the callback out of self
                let mut callback = std::mem::replace(&mut self.callback, Box::new(|_| {}));
                // Call it
                callback(self);
                // Return it to self
                self.callback = callback;

                if let Some(state) = self.state.as_mut() {
                    // Draw again
                    state.get_window().request_redraw();
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(state) = self.state.as_mut() {
                    state.resize(size);
                }
                self.size = size;
            }
            _ => (),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }

    pub fn new(position: [f32; 3], col: types::Color) -> Self {
        Self {
            position,
            color: col.into(),
        }
    }
}

pub struct RendiumBuilder {
    size: winit::dpi::PhysicalSize<u32>,
    title: String,
}

impl RendiumBuilder {
    pub fn new() -> Self {
        Self {
            size: winit::dpi::PhysicalSize::new(600, 600),
            title: "Window".to_string(),
        }
    }

    pub fn with_size(mut self, w: u32, h: u32) -> Self {
        self.size = winit::dpi::PhysicalSize::new(w, h);
        self
    }

    pub fn with_title(mut self, t: &str) -> Self {
        self.title = t.to_string();
        self
    }

    pub fn run<F: 'static + FnMut(&mut RendiumInstance) -> ()>(&self, f: F) {
        env_logger::init();

        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = RendiumInstance::new(self.size, self.title.clone(), Box::new(f));
        event_loop.run_app(&mut app).unwrap();
    }
}

pub struct RendiumDrawHandle {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    window_size: PhysicalSize<u32>,
}

impl RendiumDrawHandle {
    pub fn new(window_size: PhysicalSize<u32>) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            window_size,
        }
    }

    pub fn add_vertex(&mut self, pos: [f32; 3], col: types::Color) {
        let size = self.window_size;
        let ndc_x = (pos[0] as f32 / size.width as f32) * 2.0 - 1.0;
        let ndc_y = 1.0 - (pos[1] as f32 / size.height as f32) * 2.0;
        let ndc_pos = [ndc_x, ndc_y, pos[2]];
        self.vertices.push(Vertex::new(ndc_pos, col));
    }

    pub fn add_index(&mut self, i: u16) {
        self.indices.push(i);
    }
}

pub mod input;
mod input_wrapper;
pub mod shapes;
pub mod types;

pub fn init() -> RendiumBuilder {
    RendiumBuilder::new()
}
