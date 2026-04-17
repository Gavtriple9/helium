use anyhow::Result;
use std::sync::Arc;
use winit::window::Window;

/// Represents the state of the GPU, including the device, queue, and surface configuration.
pub struct GpuState {
    /// The GPU device handle.
    device: wgpu::Device,
    /// The GPU command queue.
    queue: wgpu::Queue,

    /// The surface to render to.
    surface: wgpu::Surface<'static>,
    /// The surface configuration.
    config: wgpu::SurfaceConfiguration,
    /// The current size of the surface.
    size: winit::dpi::PhysicalSize<u32>,
}

impl GpuState {
    /// Initializes the GPU state by creating a device, queue, and configuring the surface.
    ///
    /// # Arguments
    /// * `window` - An `Arc` to the window to render to.
    ///
    /// # Returns
    /// A `Result` containing the initialized `GpuState` or an error if initialization fails.
    pub async fn new(window: Arc<Window>) -> Result<Self> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle());

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: wgpu::Limits::default(),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        if size.width > 0 && size.height > 0 {
            surface.configure(&device, &config);
        }

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
        })
    }

    /// Resizes the surface to the new size and reconfigures it.
    ///
    /// This method should be called when the window is resized to ensure the surface matches the new size.
    ///
    /// # Arguments    
    /// * `new_size` - The new size to resize the surface to.
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Returns the current size of the surface.
    pub fn surface_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.size
    }

    /// Acquires the current surface texture for rendering.
    pub(crate) fn current_texture(&self) -> wgpu::CurrentSurfaceTexture {
        self.surface.get_current_texture()
    }

    /// Returns a reference to the GPU device.
    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    /// Returns a reference to the GPU command queue.
    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    /// Returns a reference to the surface configuration.
    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }
}
