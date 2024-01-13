use skia_safe::Canvas;
use winit::event::WindowEvent;
use crate::context::window_context::WindowContext;


pub trait UI{
    fn draw(&mut self, canvas: &Canvas);
    
    fn handle_event(&mut self, event: WindowEvent, event_context: &mut WindowContext);
}