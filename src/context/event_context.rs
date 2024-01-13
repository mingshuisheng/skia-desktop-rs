use winit::event_loop::EventLoopWindowTarget;
use winit::window::WindowId;
use crate::application::Application;
use crate::custom_event::CustomEvent;

pub struct EventContext<'a>{
    application: &'a mut Application,
    event_loop: &'a EventLoopWindowTarget<CustomEvent>,
    window_id: WindowId,
}

impl<'a> EventContext<'a> {
    pub(crate) fn new(application: &'a mut Application, event_loop: &'a EventLoopWindowTarget<CustomEvent>, window_id: WindowId) -> Self{
        Self{
            application,
            event_loop,
            window_id
        }
    }
    
    pub fn request_redraw(&mut self){
        self.application.get_mut_window(self.window_id).unwrap().request_redraw();
    }
}