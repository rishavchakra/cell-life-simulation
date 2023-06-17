use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use super::renderer::Renderer;

pub struct WindowData {
    window: Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    event_loop: EventLoop<()>,
    pub device: wgpu::Device,
    instance: wgpu::Instance,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    pub surface_config: wgpu::SurfaceConfiguration,
}

impl WindowData {
    pub async fn new(window_title: &str) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(window_title)
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        // Safety:
        // Surface needs to live as long as its window
        // safe because the state owns the surface
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                    limits: wgpu::Limits::default(),
                    label: Some("GPU Adapter device"),
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        // let surface_format = surface.get_default_config(&adapter, size.width, size.height).unwrap().format;
        // let surface_format = wgpu::TextureFormat::R32Float;
        // let surface_format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &surface_config);

        Self {
            window,
            size,
            event_loop,
            device,
            instance,
            queue,
            surface,
            surface_config,
        }
    }

    fn _resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = self.size.width;
            self.surface_config.height = self.size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }
}

pub fn run (
    WindowData {
        // destructured to allow for partial borrows
        window,
        mut size,
        event_loop,
        device,
        instance: _instance,
        queue,
        surface,
        mut surface_config,
    }: WindowData,
    mut renderer: Renderer
) {
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            let surface_texture = &surface.get_current_texture().unwrap();
            let render_command_encoder = renderer.render(&device, &surface_texture);
            queue.submit(std::iter::once(render_command_encoder.finish()));
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::Resized(new_inner_size) => {
                size = *new_inner_size;
                surface_config.width = size.width;
                surface_config.height = size.height;
                surface.configure(&device, &surface_config);
            }
            WindowEvent::ScaleFactorChanged {
                // scale_factor,
                new_inner_size,
                ..
            } => {
                size = **new_inner_size;
                surface_config.width = size.width;
                surface_config.height = size.height;
                surface.configure(&device, &surface_config);
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
