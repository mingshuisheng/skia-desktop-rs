use std::time::Duration;
use winit::window::{Window};
use crate::application::{TimerId};
use crate::context::context::Context;

pub struct WindowContext<'a> {
    application_context: Context<'a>,
    winit_window: &'a mut Window,
}

impl<'a> WindowContext<'a> {
    pub(crate) fn new(application_context: Context<'a>, winit_window: &'a mut Window) -> Self {
        Self {
            application_context,
            winit_window,
        }
    }

    pub fn request_redraw(&mut self) {
        self.winit_window.request_redraw();
    }

    pub fn start_drag(&mut self) {
        self.winit_window.drag_window().unwrap();
    }

    pub fn close_window(&mut self) {
        self.application_context.close_window(self.winit_window.id());
    }

    pub fn application(&mut self) -> &mut Context<'a> {
        &mut self.application_context
    }

    pub fn add_timer(&mut self, time: Duration) {
        let window_id = self.winit_window.id();
        self.application_context.set_timer(time, move |id, application, event_loop| {
            let window = application.remove_window(window_id);
            if let Some(mut window) = window {
                window.on_timeout(id, application, event_loop);
                application.add_window(window_id, window);
            }
        });
    }

    pub fn remove_timer(&mut self, id: TimerId) {
        self.application_context.clear_timer(id);
    }

    pub fn add_interval(&mut self, time: Duration) {
        let window_id = self.winit_window.id();
        self.application_context.set_interval(time, move |id, application, event_loop| {
            let window = application.remove_window(window_id);
            if let Some(mut window) = window {
                window.on_interval(id, application, event_loop);
                application.add_window(window_id, window);
            }
        });
    }

    pub fn remove_interval(&mut self, id: TimerId) {
        self.application_context.clear_interval(id);
    }
}