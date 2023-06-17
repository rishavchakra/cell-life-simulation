use wgpu::util::DeviceExt;

pub struct SimulationParamsBuf {
    pub params: SimulationParams,
    pub params_buf: wgpu::Buffer,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimulationParams {
    width: u32,
    height: u32,
}

impl SimulationParamsBuf {
    pub fn new(device: &wgpu::Device, params: SimulationParams) -> Self {
        let params_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Simulation Parameters Buffer"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self { params, params_buf }
    }
}

impl SimulationParams {
    pub fn new(size: &winit::dpi::PhysicalSize<u32>) -> Self {
        Self {
            width: size.width,
            height: size.height,
        }
    }
}
