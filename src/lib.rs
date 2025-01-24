use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes())
            .expect("Could not create window"));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Closing window");
                event_loop.exit();
            },
            _ => (),
        }
    }
}

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Could not create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = App::default();

    event_loop.run_app(&mut app);
}
