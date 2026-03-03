use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::{util::DeviceExt, wgc::device::queue, SurfaceConfiguration};

pub struct Bolt<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: SurfaceConfiguration,
    start_time: std::time::Instant
}
impl<'a> Bolt<'a> {
    pub async fn new(window: &'a winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface =  instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .expect("No suitable GPU adapters found");

            let (device, queue) = adapter
            .request_device(&Default::default())
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let format = surface_caps.formats[0];

        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            start_time: std::time::Instant::now()
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&Default::default());

        {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Clear Pass"),
                occlusion_query_set: None,
                timestamp_writes: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
            });
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Bolt Engine")
        .with_inner_size(winit::dpi::LogicalSize::new(1024.0, 768.0))
        .build(&event_loop)
        .unwrap();

    let mut state = pollster::block_on(Bolt::new(&window));
    let window_id = window.id();

    let shader_module = state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
    });

    #[repr(C)]
    #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
    struct Uniforms {
        time: f32,
    }

    let mut uniforms = Uniforms { time: 0.0 };

    let uniform_buffer = state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[uniforms]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
    });

    let uniform_bind_group_layout =
        state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniform Bind Group Layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None
                },
                count: None,
            }],
        });

    let uniform_bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Uniform Bind Group"),
        layout: &uniform_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: uniform_buffer.as_entire_binding(),
        }],
    });
    

    let render_pipeline_layout = state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Triangle Pipeline Layout"),
        bind_group_layouts: &[ &uniform_bind_group_layout ],
        push_constant_ranges: &[],
    });

    

    let pipeline = state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: state.config.format,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default()
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None
    });

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);




        println!("Event: {:?}", event);

        let frame = state.surface.get_current_texture().expect("No frame found");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            let elapsed = state.start_time.elapsed().as_secs_f32();
            uniforms.time = elapsed;

            state.queue.write_buffer(
                &uniform_buffer,
                0,
                bytemuck::cast_slice(&[uniforms]),
            );

            render_pass.set_pipeline(&pipeline);
            render_pass.set_bind_group(0, &uniform_bind_group, &[]);
            render_pass.draw(0..3, 0..1);
            
        }

        state.queue.submit(Some(encoder.finish()));
        frame.present();
            

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id: id,
            } if id == window_id => elwt.exit(),

            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                window_id: id,
            } if id == window_id => state.resize(size),

            _ => {}
        }
    }).unwrap();
}