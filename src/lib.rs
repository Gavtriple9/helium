use anyhow::Result;
use helium_core::App;
use winit::event_loop::{ControlFlow, EventLoop};

pub fn run_viewer() -> Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
