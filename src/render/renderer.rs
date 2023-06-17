use crate::shared::{
    shader::Shader,
    sim_params::{SimulationParams, SimulationParamsBuf},
    texture::Texture,
};
use wgpu::util::DeviceExt;

pub struct Renderer {
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    _render_shader: Shader,
}

impl Renderer {
    pub fn new(
        device: &wgpu::Device,
        cell_texture: &Texture,
        sim_params: &SimulationParamsBuf,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let vertex_data = [
            Vertex::new([-1, -1], [0, 0]),
            Vertex::new([1, -1], [1, 0]),
            Vertex::new([1, 1], [1, 1]),
            Vertex::new([-1, 1], [0, 1]),
        ];
        let index_data: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let render_shader =
            Shader::new("src/render/render.wgsl", device).expect("Shader compilation failed!");

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Render Bind Group Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::ReadOnly,
                        format: cell_texture.texture_format,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<SimulationParams>() as _,
                        ),
                    },
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    count: None,
                },
            ],
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Render Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&cell_texture.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: sim_params.params_buf.as_entire_binding(),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &render_shader.module,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()], // buffers
            },
            fragment: Some(wgpu::FragmentState {
                module: &render_shader.module,
                entry_point: "fs_main",
                targets: &[Some(surface_config.format.into())],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                strip_index_format: None,
                conservative: false,
            },
            depth_stencil: None,
            multiview: None,
            multisample: wgpu::MultisampleState::default(),
        });

        Self {
            vertex_buf,
            index_buf,
            bind_group,
            pipeline,
            _render_shader: render_shader,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        surface_texture: &wgpu::SurfaceTexture,
    ) -> wgpu::CommandEncoder {
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            depth_stencil_attachment: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            })],
        });
        render_pass.push_debug_group("Setup for drawing");
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        render_pass.pop_debug_group();

        render_pass.push_debug_group("Ready to draw");
        render_pass.insert_debug_marker("Drawing");
        render_pass.draw_indexed(0..6, 0, 0..1);
        render_pass.pop_debug_group();
        drop(render_pass);
        drop(view);

        command_encoder
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    pos: [f32; 4],
    uv_coord: [f32; 2],
}

impl Vertex {
    fn new(pos: [i8; 2], uv: [u8; 2]) -> Self {
        Vertex {
            pos: [pos[0] as f32, pos[1] as f32, 0.0, 1.0],
            uv_coord: [uv[0] as f32, uv[1] as f32],
        }
    }

    const ATTR_ARRAY: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4,
        1 => Float32x2,
    ];

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<SimulationParams>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Vertex::ATTR_ARRAY,
        }
    }
}
