use wgpu::util::DeviceExt;

pub struct SimulationState {
    pub generation: usize,
    // Two buffers to alternate rendering and referencing
    cell_buffers: [wgpu::Buffer; 2],
    trail_buffers: [wgpu::Buffer; 2],

    // compute_pipeline: wgpu::ComputePipeline,
    // bind_groups: Vec<wgpu::BindGroup>,
}

pub struct SimulationParams {
    width: u32,
    height: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimulationCell {
    status: u8,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct SimulationTrail {
    val: f32,
}

impl SimulationState {
    pub fn new(device: &wgpu::Device, initial_state: Vec<SimulationCell>) -> Self {
        let size = initial_state.len() as u64;
        let cell_buffers = [
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Simulation Cell Buffer 0"),
                contents: bytemuck::cast_slice(&initial_state),
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            }),
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Simulation Cell Buffer 1"),
                contents: bytemuck::cast_slice(&initial_state),
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
            }),
        ];

        let trail_buffers = [
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Simulation Trail Buffer 0"),
                size: std::mem::size_of::<SimulationTrail>() as u64 * size,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: true,
            }),
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Simulation Trail Buffer 1"),
                size: std::mem::size_of::<SimulationTrail>() as u64 * size,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: true,
            }),
        ];



        Self {
            generation: 0,
            cell_buffers,
            trail_buffers,
        }
    }

    pub fn step(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        let buf_step = self.generation % 2;
        // let cell_buf = self.cell_buffers[buf_step];
        // let trail_buf = self.trail_buffers[buf_step];
        self.generation += 1;
    }
}
