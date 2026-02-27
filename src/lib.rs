use anyhow::Result;
use helium_core::App;
use winit::event_loop::{ControlFlow, EventLoop};

#[cfg(any(target_os = "ios", target_os = "macos"))]
use std::sync::atomic::{AtomicBool, Ordering};

#[unsafe(no_mangle)]
pub extern "C" fn helium_ffi_ping() -> u32 {
    1
}

pub fn run_helium() -> Result<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
pub extern "system" fn Java_dev_helium_viewer_MainActivity_pingFromRust(
    _env: *mut core::ffi::c_void,
    _class: *mut core::ffi::c_void,
) -> i32 {
    1
}

#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
pub extern "C" fn helium_start_ios_viewer() -> i32 {
    static STARTED: AtomicBool = AtomicBool::new(false);

    if STARTED.swap(true, Ordering::SeqCst) {
        return 1;
    }

    match std::panic::catch_unwind(run_helium) {
        Ok(Ok(())) => 0,
        Ok(Err(error)) => {
            eprintln!("helium_start_ios_viewer failed: {error}");
            -1
        }
        Err(_) => {
            eprintln!("helium_start_ios_viewer panicked");
            -2
        }
    }
}

#[cfg(target_os = "macos")]
#[unsafe(no_mangle)]
pub extern "C" fn helium_start_macos_viewer() -> i32 {
    static STARTED: AtomicBool = AtomicBool::new(false);

    if STARTED.swap(true, Ordering::SeqCst) {
        return 1;
    }

    match std::panic::catch_unwind(run_helium) {
        Ok(Ok(())) => 0,
        Ok(Err(error)) => {
            eprintln!("helium_start_macos_viewer failed: {error}");
            -1
        }
        Err(_) => {
            eprintln!("helium_start_macos_viewer panicked");
            -2
        }
    }
}
