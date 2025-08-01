use std::sync::Arc;
use pollster::block_on;
use crate::core::render::state::State;
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

pub struct App {
    state: Option<State>,
    window: Option<Arc<Window>>
}

impl Default for App {
    fn default() -> Self {
        Self { state: None, window: None }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes()
                .with_title("Rustcraft Alpha-1.0")
                .with_resizable(false);
    
            let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
            let state = block_on(State::new(Arc::clone(&window)));
                
            self.state = Some(state);
            self.window = Some(window);
        }
    }

    fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
        if Some(window_id) != self.window.as_ref().map(|w| w.id()) {
            return;
        }

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