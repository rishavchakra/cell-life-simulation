struct SimulationState {
    generation: usize,
    buffers: [wgpu::Buffer; 2],
}

impl SimulationState {
    pub fn new(device: &wgpu::Device, initial_state: Vec<f32>) -> Self {
        let buffers = [
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some( "Simulation Buffer 0" ),
                size: initial_state.len() as u64,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: true,
            }),
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some( "Simulation Buffer 1" ),
                size: initial_state.len() as u64,
                usage: wgpu::BufferUsages::STORAGE
                    | wgpu::BufferUsages::COPY_DST
                    | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: true,
            }),
        ];

        Self {
            generation: 0,
            buffers,
        }
    }

    pub fn next_generation(&mut self) {
        self.generation += 1;
    }
}
