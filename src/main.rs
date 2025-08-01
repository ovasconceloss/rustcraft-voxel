use winit::{
    event_loop::{ControlFlow, EventLoop}, 
};
use std::time::Instant;
use rustcraft::core::window::window::App;

fn main() {    
    let before_instant: Instant = Instant::now();

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

    println!("Elapsed Time: {:.2?}", before_instant.elapsed());
}