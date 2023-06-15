use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod gpu_state;
mod simulation;

pub async fn run() {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut gpu_state = gpu_state::GpuState::new(window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == gpu_state.window().id() => {
            gpu_state.update();
            match gpu_state.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => gpu_state.resize(*gpu_state.size()),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(wgpu::SurfaceError::Outdated) => {
                    eprintln!("Underlying surface has changed, swap chain must be updated");
                    *control_flow = ControlFlow::Exit;
                }
                Err(wgpu::SurfaceError::Timeout) => {
                    eprintln!("Timeout while trying to get next frame");
                    *control_flow = ControlFlow::Exit;
                }
                // Err(e) => eprintln!("{}", e.to_string()),
            }
        }
        Event::MainEventsCleared => {
            gpu_state.window().request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == gpu_state.window().id() && !gpu_state.input(event) => match event {
            WindowEvent::Resized(new_inner_size) => {
                gpu_state.resize(*new_inner_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                gpu_state.resize(**new_inner_size);
            }
            // Window close event or Escape key pressed: Exit
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    })
}
