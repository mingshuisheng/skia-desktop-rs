use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window};
use crate::application::Application;
use crate::context::context::Context;
use crate::custom_event::CustomEvent;

pub struct WindowContext<'a> {
    application_context: Context<'a>,
    winit_window: &'a mut Window,
}

impl<'a> WindowContext<'a> {
    pub(crate) fn new(application: &'a mut Application, event_loop: &'a EventLoopWindowTarget<CustomEvent>, winit_window: &'a mut Window) -> Self {
        Self {
            application_context: Context::new(application, event_loop),
            winit_window,
        }
    }

    pub fn request_redraw(&mut self) {
        self.winit_window.request_redraw();
    }

    pub fn start_drag(&mut self) {
        self.winit_window.drag_window().unwrap();
    }

    pub fn close_window(&mut self){
        self.application_context.close_window(self.winit_window.id());
    }

    pub fn application(&mut self) -> &mut Context<'a>{
        &mut self.application_context
    }
}