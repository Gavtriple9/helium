use crate::GpuState;

pub struct Frame {
    pub output: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

impl Frame {
    pub fn begin(gpu: &mut GpuState) -> Option<Self> {
        let output = match gpu.current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                let size = gpu.surface_size();
                gpu.resize(size);
                return None;
            }
            other => {
                tracing::error!("Failed to acquire surface texture: {other:?}");
                return None;
            }
        };
        let view = output.texture.create_view(&Default::default());
        let encoder = gpu
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        Some(Self {
            output,
            view,
            encoder,
        })
    }

    pub fn submit(self, gpu: &GpuState) {
        gpu.queue().submit(Some(self.encoder.finish()));
        self.output.present();
    }
}
