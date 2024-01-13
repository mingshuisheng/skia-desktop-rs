use std::ops::{Deref, DerefMut};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window};
use crate::application::Application;
use crate::context::context::Context;
use crate::custom_event::CustomEvent;

pub struct WindowContext<'a> {
    application_context: Context<'a>,
    winit_window: &'a mut Window
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
}

impl<'a> Deref for WindowContext<'a>{
    type Target = Context<'a>;

    fn deref(&self) -> &Self::Target {
        &self.application_context
    }
}

impl<'a> DerefMut for WindowContext<'a>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.application_context
    }
}