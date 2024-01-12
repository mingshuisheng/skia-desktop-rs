use std::collections::HashMap;
use winit::event::{Event, StartCause};
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopWindowTarget};
use winit::window::{WindowBuilder, WindowId};
use crate::custom_event::CustomEvent;
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
}

pub struct EventHandler {
    handle_init: Option<Box<dyn Fn(Controller)>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            handle_init: None
        }
    }

    pub fn set_init_handler(&mut self, handle_init: Option<Box<dyn Fn(Controller)>>) {
        self.handle_init = handle_init
    }
    fn on_init(&self, controller: Controller) {
        if let Some(handler) = &self.handle_init {
            handler(controller);
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Controller<'a> {
    application: &'a mut Application,
    event_loop: &'a EventLoopWindowTarget<CustomEvent>,
}

impl<'a> Controller<'a> {
    fn new(application: &'a mut Application, event_loop: &'a EventLoopWindowTarget<CustomEvent>) -> Self {
        Self {
            application,
            event_loop,
        }
    }

    pub fn new_window(&mut self, wb: WindowBuilder) -> Option<&mut Window> {
        let window = Window::new(wb, self.event_loop);
        let window_id = window.id();
        self.application.window_map.insert(window_id, window);
        self.application.window_map.get_mut(&window_id)
    }

    pub fn get_mut_window(&mut self, window_id: WindowId) -> Option<&mut Window> {
        self.application.window_map.get_mut(&window_id)
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
                    if let Some(window) = application.window_map.remove(&window_id) {
                        window.on_close();
                    }
                    if  application.window_map.len() == 0 {
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
