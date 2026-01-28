use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
mod draw;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

#[derive(Default)]
struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Ray Tracer")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

        // SurfaceTexture needs to own a reference to the window.
        // Arc::clone(&window) creates a new Arc that points to the same window.
        let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, Arc::clone(&window));

        // Pixels::new will return a Pixels<'static> because surface_texture owns the Arc.
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    draw::draw(pixels);

                    if let Err(err) = pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        event_loop.exit();
                        return;
                    }
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}

