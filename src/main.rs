mod compute;
mod render;
mod shared;

use render::{renderer, window};
use shared::{
    sim_params::{SimulationParams, SimulationParamsBuf},
    texture,
};

#[tokio::main]
async fn main() {
    let window = window::WindowData::new("Cells").await;
    let cell_texture =
        texture::Texture::new(&window.device, &window.size, wgpu::TextureFormat::R32Float);
    let simulation_params =
        SimulationParamsBuf::new(&window.device, SimulationParams::new(&window.size));
    let renderer = renderer::Renderer::new(
        &window.device,
        &cell_texture,
        &simulation_params,
        &window.surface_config,
    );
    // Can access through closure arguments the window data
    // needs to be passed shared and simulation arguments by reference
    window::run(window, renderer);
}
