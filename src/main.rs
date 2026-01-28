use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
mod draw;

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    cursor_pos: (f64, f64),
    width: u32,
    height: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            pixels: None,
            cursor_pos: (0.0, 0.0),
            width: 800,
            height: 600,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attrs = Window::default_attributes()
            .with_title("Ray Tracer")
            .with_inner_size(winit::dpi::LogicalSize::new(self.width, self.height));

        let window = Arc::new(event_loop.create_window(window_attrs).unwrap());

        // SurfaceTexture needs to own a reference to the window.
        // Arc::clone(&window) creates a new Arc that points to the same window.
        let surface_texture = SurfaceTexture::new(self.width, self.height, Arc::clone(&window));

        // Pixels::new will return a Pixels<'static> because surface_texture owns the Arc.
        let pixels = Pixels::new(self.width, self.height, surface_texture).unwrap();

        self.window = Some(window);

        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(new_size) => {
                self.width = new_size.width;
                self.height = new_size.height;

                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(new_size.width, new_size.height);
                    let _ = pixels.resize_buffer(new_size.width, new_size.height);
                }

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_pos = (position.x, position.y);
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }

            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                    println!("Left click detected at: {:?}", self.cursor_pos);
                }
            }

            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    draw::draw(pixels, self.cursor_pos, self.width, self.height);

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
