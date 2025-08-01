use winit::{
    window::Window,
    event::WindowEvent, 
    application::ApplicationHandler, 
    event_loop::{ControlFlow, EventLoop}, 
};
use std::time::Instant;

#[derive(Default)]
struct App {
    window: Option<Window>
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Rustcraft Alpha-1.0")
            .with_resizable(false);
            
        self.window = Some(event_loop.create_window(window_attributes).unwrap());
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => ()
        }
    }
}

fn main() {    
    let before_instant: Instant = Instant::now();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

    println!("Elapsed Time: {:.2?}", before_instant.elapsed());
}