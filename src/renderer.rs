use crate::{ simulation::SimulationState, gpu_state::GpuState };

pub struct Renderer {
    pub sim_state: SimulationState,
    pub gpu_state: GpuState,
}

impl Renderer {
    pub async fn new(window: winit::window::Window) -> Self {
        let gpu_state = GpuState::new(window).await;
        let sim_state = SimulationState::new(&gpu_state.device, vec![]);

        Self {
            gpu_state,
            sim_state,
        }
    }

    pub fn sim_step(&mut self) {
        let buf_step = self.sim_state.generation % 2;
        self.sim_state.generation += 1;
    }

    pub fn render(&self) {

    }
}
