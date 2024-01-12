use std::collections::HashMap;
use winit::event::{Event, StartCause};
use winit::event_loop::{EventLoop, EventLoopBuilder};
use winit::window::WindowId;
use crate::controller::Controller;
use crate::custom_event::CustomEvent;
use crate::event_handler::EventHandler;
use crate::window::Window;

pub struct Application {
    window_map: HashMap<WindowId, Window>,
}

impl Application {
    pub fn new() -> Self {
        Self {
            window_map: HashMap::new(),
        }
    }

    pub fn add_window(&mut self, window_id: WindowId, window: Window) -> Option<&mut Window> {
        self.window_map.insert(window_id, window);
        self.window_map.get_mut(&window_id)
    }

    pub fn remove_window(&mut self, window_id: WindowId) -> Option<Window> {
        self.window_map.remove(&window_id)
    }

    pub fn get_mut_window(&mut self, window_id: WindowId) -> Option<&mut Window> {
        self.window_map.get_mut(&window_id)
    }

    pub fn count_window(&mut self) -> usize {
        self.window_map.len()
    }
}

pub fn launch(event_handler: EventHandler) {
    let event_loop: EventLoop<CustomEvent> = EventLoopBuilder::with_user_event().build().unwrap();
    let mut application = Application::new();

    event_loop.run(move |event, event_loop| {
        match event {
            Event::NewEvents(StartCause::Init) => {
                let controller = Controller::new(&mut application, event_loop);
                event_handler.on_init(controller);
            }
            Event::WindowEvent { event, window_id } => {
                if let winit::event::WindowEvent::CloseRequested = event {
                    if let Some(window) = application.remove_window(window_id) {
                        window.on_close();
                    }
                    if application.count_window() == 0 {
                        event_loop.exit();
                    }
                }
                if let Some(window) = application.window_map.get_mut(&window_id) {
                    window.handle_event(event)
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(_) => {}
            Event::Suspended => {}
            Event::Resumed => {}
            Event::AboutToWait => {}
            Event::LoopExiting => {}
            Event::MemoryWarning => {}
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {}
            Event::NewEvents(StartCause::WaitCancelled { .. }) => {}
            Event::NewEvents(StartCause::Poll) => {}
        }
    }).unwrap();
}
