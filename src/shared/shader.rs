use anyhow::Result;

pub struct Shader {
    pub module: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(source: &str, device: &wgpu::Device) -> Result<Self> {
        let shader_str = std::fs::read_to_string(source)?;

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(shader_str.into()),
        });

        Ok(Self { module })
    }
}
