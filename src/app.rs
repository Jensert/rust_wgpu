use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowId},
};

use crate::State;

#[derive(Default)]
pub struct App {
    state: Option<State>,
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("Hello WGPU!"))
            .expect("Failed to create window");
        self.state = Some(State::new(window));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let window = self.state.as_ref().unwrap().window();

        if window.id() == window_id && !self.state.as_mut().unwrap().input(&event) {
            match event {
                WindowEvent::Resized(size) => {
                    println!("Resizing window");
                    self.state.as_mut().unwrap().resize(size);
                }

                WindowEvent::RedrawRequested => {
                    self.state.as_mut().unwrap().window().request_redraw();

                    self.state.as_mut().unwrap().update();

                    match self.state.as_mut().unwrap().render() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Rendering failed: {:?}", e);
                        }
                    }
                }

                WindowEvent::KeyboardInput { event, .. } => {
                    if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                        println!("Escape pressed!");
                        event_loop.exit();
                    }
                }

                WindowEvent::CloseRequested => {
                    println!("Closing window");
                    event_loop.exit();
                }
                _ => (),
            }
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        use winit::event::DeviceEvent;

        match event {
            DeviceEvent::MouseMotion { delta: (dx, dy) } => {
                if let Some(state) = self.state.as_mut() {
                    state
                        .camera_controller
                        .process_mouse_motion(&mut state.camera, dx, dy);
                }
            }
            _ => {}
        }
    }
}
