use winit::event_loop::EventLoopWindowTarget;
use winit::window::{WindowBuilder, WindowId};
use crate::application::Application;
use crate::custom_event::CustomEvent;
use crate::ui::UI;
use crate::window::Window;

pub struct Controller<'a> {
    application: &'a mut Application,
    event_loop: &'a EventLoopWindowTarget<CustomEvent>,
}

impl<'a> Controller<'a> {
    pub(crate) fn new(application: &'a mut Application, event_loop: &'a EventLoopWindowTarget<CustomEvent>) -> Self {
        Self {
            application,
            event_loop,
        }
    }

    pub fn new_window(&mut self, wb: WindowBuilder,  ui: impl UI + 'static) -> Option<&mut Window> {
        let window = Window::new(wb, self.event_loop, ui);
        let window_id = window.id();
        self.application.add_window(window_id, window)
    }

    pub fn get_mut_window(&mut self, window_id: WindowId) -> Option<&mut Window> {
        self.application.get_mut_window(window_id)
    }
}
