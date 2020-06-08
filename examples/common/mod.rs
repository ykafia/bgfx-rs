// Copyright (c) 2015-2016, Johan Sköld.
// License: http://opensource.org/licenses/ISC

extern crate bgfx;
extern crate bgfx_sys;
extern crate glutin;
extern crate libc;

use bgfx::{Bgfx, PlatformData, RenderFrame};

use glutin::{Api, GlRequest, Window, WindowBuilder};

use std;
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread;
use std::time::Duration;
use self::bgfx_sys::bgfx_texture_format_BGFX_TEXTURE_FORMAT_RGB8;

/// Events received by the main thread, sent by the render thread.
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Event {
    /// Window close event.
    Close,

    /// Window size event.
    Size(u16, u16),
}

/// Event queue for communicating with the render thread.
pub struct EventQueue {
    event_rx: Receiver<Event>,
}

impl EventQueue {

    /// Handles events received from the render thread. If there are no events to process, returns
    /// instantly.
    ///
    /// Returns `true` if the app should exit.
    pub fn handle_events(&self,
                         bgfx: &Bgfx,
                         width: &mut u16,
                         height: &mut u16,
                         reset: bgfx::ResetFlags)
                         -> bool {
        let mut should_close = false;

        while let Ok(result) = self.event_rx.try_recv() {
            match result {
                Event::Close => should_close = true,
                Event::Size(w, h) => {
                    *width = w;
                    *height = h;
                    bgfx.reset(w, h, reset, bgfx_texture_format_BGFX_TEXTURE_FORMAT_RGB8 /* TODO format */);
                }
            }
        }

        should_close
    }

}

/// Process window events on the render thread.
fn process_events(window: &Window, event_tx: &Sender<Event>) -> bool {
    let mut should_close = false;

    for event in window.poll_events() {
        match event {
            glutin::Event::Closed => {
                should_close = true;
                event_tx.send(Event::Close).unwrap();
            }
            glutin::Event::Resized(w, h) => {
                event_tx.send(Event::Size(w as u16, h as u16)).unwrap();
            }
            _ => {}
        }
    }

    should_close
}

/// Loads the contents of a file and returns it.
fn load_file(name: &str) -> Vec<u8> {
    let mut data = Vec::new();
    let mut file = File::open(name).unwrap();
    file.read_to_end(&mut data).unwrap();
    data
}

/// Loads the two given shaders from disk, and creates a program containing the loaded shaders.
#[allow(dead_code)]
pub fn load_program<'a, 'b>(bgfx: &'a Bgfx,
                            vsh_name: &'b str,
                            fsh_name: &'b str)
                            -> bgfx::Program<'a> {
    let renderer = bgfx.get_renderer_type();
    let exe_path = env::current_exe().unwrap();
    let exe_stem = exe_path.file_stem().unwrap();
    let assets_path = format!("examples/assets/{}", exe_stem.to_str().unwrap());
    let vsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, vsh_name);
    let fsh_path = format!("{}/{:?}/{}.bin", assets_path, renderer, fsh_name);
    let vsh_mem = bgfx::Memory::copy(bgfx, &load_file(&vsh_path));
    let fsh_mem = bgfx::Memory::copy(bgfx, &load_file(&fsh_path));
    let vsh = bgfx::Shader::new(vsh_mem);
    let fsh = bgfx::Shader::new(fsh_mem);

    bgfx::Program::new(vsh, fsh)
}

/// Set the platform data to be used by BGFX.
#[cfg(target_os = "linux")]
fn init_bgfx_platform(window: &Window) {
    PlatformData::new()
        .display(unsafe {window.platform_display() as *mut std::os::raw::c_void })
        .window(unsafe {window.platform_window() as *mut std::os::raw::c_void })
        .apply()
        .unwrap();
}

/// Set the platform data to be used by BGFX.
#[cfg(not(target_os = "linux"))]
fn init_bgfx_platform(window: &Window) {
    PlatformData::new()
        .window(unsafe {window.platform_window() as *mut std::os::raw::c_void })
        .apply()
        .unwrap();
}

pub fn run_example<M>(width: u16, height: u16, main: M)
    where M: Send + 'static + FnOnce(EventQueue)
{
    let window = WindowBuilder::new()
                     .with_dimensions(width as u32, height as u32)
                     .with_gl(GlRequest::Specific(Api::OpenGl, (3, 1)))
                     .with_title(String::from("BGFX"))
                     .build()
                     .expect("Failed to create window");

    unsafe {
        window.make_current().unwrap();
    }

    // Create the channel used for communication between the main and render threads.
    let (event_tx, event_rx) = channel::<Event>();

    // Set the platform data for BGFX to use.
    init_bgfx_platform(&window);

    // Initialize this thread as the render thread by pumping it once *before* calling bgfx::init.
    bgfx::render_frame(16 /* TODO */);

    // Spawn a new thread to use as the main thread.
    let main_thread = thread::spawn(move || {
        main(EventQueue { event_rx: event_rx });
    });

    // Pump window events until the window is closed.
    while !process_events(&window, &event_tx) {
        bgfx::render_frame(16 /* TODO */);
    }

    // Pump the render thread until the main thread has shut down.
    while bgfx::render_frame(16 /* TODO */) != RenderFrame::NoContext {
        thread::sleep(Duration::from_millis(1));
    }

    main_thread.join().unwrap();
}
