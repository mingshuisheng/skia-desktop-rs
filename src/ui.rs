use skia_safe::Canvas;
use crate::context::window_context::WindowContext;
use crate::event::ui_event::UIEvent;


pub trait UI{
    fn draw(&mut self, canvas: &Canvas);
    
    fn handle_event(&mut self, event: UIEvent, event_context: &mut WindowContext);
}