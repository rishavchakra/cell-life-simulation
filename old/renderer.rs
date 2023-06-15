use crate::{ simulation::{ SimulationState, SimulationParams }, gpu_state::GpuState };
use wgpu::util::DeviceExt;

pub struct Renderer {
    pub sim_state: SimulationState,
    pub gpu_state: GpuState,

    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,

    vertex_data: [Vertex; 4],
    index_data: [u16; 6],
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f32; 4],
    uv_coord: [f32; 2],
}

impl Renderer {
    pub async fn new(window: winit::window::Window, texture: &Texture) -> Self {
        let gpu_state = GpuState::new(window).await;
        let sim_state = SimulationState::new(&gpu_state.device, vec![]);

        let shader = gpu_state.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Rendering Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/render.wgsl").into()),
        });

        let cell_bind_group_layout = gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Layout"),
            entries: &[]
        });

        let pipeline_layout = gpu_state.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[], // add bind group layouts here
            push_constant_ranges: &[],
        });

        let pipeline = gpu_state.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: gpu_state.config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                polygon_mode: wgpu::PolygonMode::Fill,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                strip_index_format: None,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multiview: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        let vertex_data = [
            Vertex {
                pos: [-1.0, -1.0, 1.0, 1.0],
                uv_coord: [0.0, 0.0],
            },
            Vertex {
                pos: [1.0, -1.0, 1.0, 1.0],
                uv_coord: [0.0, 0.0],
            },
            Vertex {
                pos: [1.0, 1.0, 1.0, 1.0],
                uv_coord: [0.0, 0.0],
            },
            Vertex {
                pos: [-1.0, 1.0, 1.0, 1.0],
                uv_coord: [0.0, 0.0],
            },
        ];
        let index_data = [
            0, 1, 2, 2, 3, 0
        ];

        let vertex_buf = gpu_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buf = gpu_state.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let bind_group_layout = gpu_state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Group Layout Descriptor"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
                        format: wgpu::TextureFormat::R32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<SimulationParams>() as _
                        ),
                    },
                    count: None,
                },
            ],
        });
        let bind_group = gpu_state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::
                }
            ]
        })


        Self {
            gpu_state,
            sim_state,
            pipeline,
            vertex_data,
            index_data,
            vertex_buf,
            index_buf,
        }
    }

    pub fn render(&mut self, view: &wgpu::TextureView) {
        let mut encoder = self
            .gpu_state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                }
            })],
            depth_stencil_attachment: None,
        });
        render_pass.push_debug_group("Setup for drawing");
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        render_pass.pop_debug_group();

        render_pass.push_debug_group("Ready to draw");
        render_pass.draw_indexed(0..self.index_data.len() as u32, 0, 0..1);
    }
}
