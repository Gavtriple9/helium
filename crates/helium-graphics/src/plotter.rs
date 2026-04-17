use crate::{Frame, GpuState};
use wgpu::util::DeviceExt;

/// A vertex representing a point in 2D space, used for plotting.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
}

/// Generates a vector of vertices representing a sine wave based on the given amplitude, frequency, and phase.
fn generate_vertices(amp: f32, freq: f32, phase: f32) -> Vec<Vertex> {
    (0..1000)
        .map(|i| {
            let x = i as f32 / 999.0;
            let y = ((x * freq) + phase.to_radians()).sin() * amp;
            Vertex {
                position: [x * 2.0 - 1.0, y],
            }
        })
        .collect()
}

/// A simple plot renderer that generates vertices for a sine wave and renders them using a shader.
pub struct PlotRenderer {
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    color_bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl PlotRenderer {
    /// Creates a new `PlotRenderer` by initializing the vertex buffer, color bind group, and render pipeline.
    pub fn new(gpu: &GpuState) -> Self {
        let device = gpu.device();
        let queue = gpu.queue();

        let initial_color: [f32; 4] = [0.2, 1.0, 0.8, 1.0];
        let color_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Triangle Color Buffer"),
            size: std::mem::size_of::<[f32; 4]>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&color_buffer, 0, bytemuck::cast_slice(&initial_color));

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        let color_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Color Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Color Bind Group"),
            layout: &color_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: color_buffer.as_entire_binding(),
            }],
        });

        let vertices = generate_vertices(1.0, 5.0, 0.0);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[Some(&color_bind_group_layout)],
                immediate_size: 0,
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        format: wgpu::VertexFormat::Float32x2,
                        offset: 0,
                        shader_location: 0,
                    }],
                }],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu.config().format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::LineStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
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
            multiview_mask: None,
            cache: None,
        });

        Self {
            vertex_buffer,
            vertex_count: vertices.len() as u32,
            color_bind_group,
            pipeline,
        }
    }

    /// Updates the vertex buffer with new vertices generated from the given amplitude, frequency, and phase.
    pub fn update(&mut self, gpu: &GpuState, amp: f32, freq: f32, phase: f32) {
        let vertices = generate_vertices(amp, freq, phase);
        gpu.queue()
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&vertices));
    }

    /// Renders the plot using the provided render pass, setting the pipeline, bind group, and vertex buffer before drawing.
    pub fn render(&self, pass: &mut wgpu::RenderPass<'_>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.color_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.draw(0..self.vertex_count, 0..1);
    }

    /// Renders the plot to the given frame by beginning a render pass, calling the `render` method, and then submitting the commands.
    pub fn render_to_frame(&self, frame: &mut Frame) {
        let mut pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Plot Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
                multiview_mask: None,
            });
        self.render(&mut pass);
    }
}
