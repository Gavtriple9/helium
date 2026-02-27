use crate::{gpu::GpuState, plotter::PlotRenderer};
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowAttributes, WindowId},
};

pub struct App {
    window: Option<Arc<Window>>,
    gpu: Option<GpuState>,
    plot: Option<PlotRenderer>,

    egui_ctx: egui::Context,
    egui_state: Option<egui_winit::State>,
    egui_renderer: Option<egui_wgpu::Renderer>,

    amplitude: f32,
    frequency: f32,
    phase: f32,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            gpu: None,
            plot: None,
            egui_ctx: egui::Context::default(),
            egui_state: None,
            egui_renderer: None,
            amplitude: 0.5,
            frequency: 8.0,
            phase: 0.0,
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let window = self.window.as_ref().expect("window should be initialized");
        let gpu = self.gpu.as_mut().expect("gpu should be initialized");
        let plot = self
            .plot
            .as_mut()
            .expect("plot renderer should be initialized");

        if gpu.surface_size().width == 0 || gpu.surface_size().height == 0 {
            return Ok(());
        }

        plot.update(gpu, self.amplitude, self.frequency, self.phase);

        let output = gpu.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let mut encoder = gpu
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Plot Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            plot.render(&mut pass);
        }

        let raw_input = self
            .egui_state
            .as_mut()
            .expect("egui state should be initialized")
            .take_egui_input(window);

        let full_output = self.egui_ctx.run(raw_input, |ctx| {
            egui::Window::new("Controls").show(ctx, |ui| {
                ui.add(egui::Slider::new(&mut self.amplitude, -1.0..=1.0).text("Amplitude"));
                ui.add(egui::Slider::new(&mut self.frequency, 1.0..=100.0).text("Frequency"));
                ui.add(egui::Slider::new(&mut self.phase, 0.0..=360.0).text("Phase"));
            });
        });

        self.egui_state
            .as_mut()
            .expect("egui state should be initialized")
            .handle_platform_output(window, full_output.platform_output.clone());

        let pixels_per_point = window.scale_factor() as f32;
        let paint_jobs = self
            .egui_ctx
            .tessellate(full_output.shapes, pixels_per_point);

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [gpu.surface_size().width, gpu.surface_size().height],
            pixels_per_point,
        };

        let egui_renderer = self
            .egui_renderer
            .as_mut()
            .expect("egui renderer should be initialized");

        for (texture_id, image_delta) in &full_output.textures_delta.set {
            egui_renderer.update_texture(
                gpu.get_device().as_ref(),
                gpu.get_queue().as_ref(),
                *texture_id,
                image_delta,
            );
        }

        egui_renderer.update_buffers(
            gpu.get_device().as_ref(),
            gpu.get_queue().as_ref(),
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        {
            let mut ui_pass = encoder
                .begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("egui Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                })
                .forget_lifetime();

            egui_renderer.render(&mut ui_pass, &paint_jobs, &screen_descriptor);
        }

        for texture_id in &full_output.textures_delta.free {
            egui_renderer.free_texture(texture_id);
        }

        gpu.get_queue().submit(Some(encoder.finish()));
        output.present();

        window.request_redraw();
        Ok(())
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("helium"))
                .unwrap(),
        );

        let gpu = pollster::block_on(GpuState::new(window.clone()))
            .expect("Should be able to create GPU device");
        let plot = PlotRenderer::new(&gpu);

        let egui_state = egui_winit::State::new(
            self.egui_ctx.clone(),
            egui::ViewportId::ROOT,
            window.as_ref(),
            None,
            None,
            None,
        );

        let options = egui_wgpu::RendererOptions {
            ..Default::default()
        };
        let egui_renderer =
            egui_wgpu::Renderer::new(gpu.get_device().as_ref(), gpu.get_config().format, options);

        self.egui_state = Some(egui_state);
        self.egui_renderer = Some(egui_renderer);
        self.plot = Some(plot);
        self.gpu = Some(gpu);
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let window = self.window.as_ref().expect("window should be initialized");
        let gpu = self.gpu.as_mut().expect("gpu should be initialized");

        if self
            .egui_state
            .as_mut()
            .expect("egui state should be initialized")
            .on_window_event(window, &event)
            .consumed
        {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => gpu.resize(size),
            WindowEvent::RedrawRequested => match self.render() {
                Ok(()) => {}
                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                    if let Some(gpu) = self.gpu.as_mut() {
                        let size = gpu.surface_size();
                        gpu.resize(size);
                    }
                }
                Err(wgpu::SurfaceError::Timeout) => {}
                Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                Err(wgpu::SurfaceError::Other) => {}
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
