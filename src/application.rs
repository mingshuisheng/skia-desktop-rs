use std::collections::HashMap;
use winit::event::{Event, StartCause};
use winit::event_loop::{EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
use winit::window::WindowId;
use crate::context::context::Context;
use crate::custom_event::CustomEvent;
use crate::event_handler::EventHandler;
use crate::window::Window;

pub struct Application {
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    window_map: HashMap<WindowId, Window>,
}

impl Application {
    pub fn new(event_loop_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            event_loop_proxy,
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

    pub fn request_close_window(&mut self, window_id: WindowId) {
        self.event_loop_proxy.send_event(CustomEvent::CloseWindow(window_id)).unwrap();
    }
    pub fn request_exit(&mut self){
        self.event_loop_proxy.send_event(CustomEvent::Exit).unwrap();
    }

    fn do_close_window(&mut self, event_loop: &EventLoopWindowTarget<CustomEvent>, window_id: WindowId) {
        if let Some(window) = self.remove_window(window_id) {
            window.on_close();
        }
        if self.count_window() == 0 {
            event_loop.exit();
        }
    }
}

pub fn launch(event_handler: EventHandler) {
    let event_loop: EventLoop<CustomEvent> = EventLoopBuilder::with_user_event().build().unwrap();
    let event_loop_proxy = event_loop.create_proxy();
    let mut application = Application::new(event_loop_proxy);


    event_loop.run(move |event, event_loop| {
        match event {
            Event::NewEvents(StartCause::Init) => {
                let mut context = Context::new(&mut application, event_loop);
                event_handler.on_init(&mut context);
            }
            Event::WindowEvent { event, window_id } => {
                if let winit::event::WindowEvent::CloseRequested = event {
                    application.do_close_window(event_loop, window_id);
                }
                if let Some(mut window) = application.window_map.remove(&window_id) {
                    window.handle_event(event, &mut application, event_loop);
                    application.window_map.insert(window_id, window);
                }
            }
            Event::DeviceEvent { .. } => {}
            Event::UserEvent(event) => {
                match event {
                    CustomEvent::CloseWindow(window_id) => {
                        application.do_close_window(event_loop, window_id);
                    }
                    CustomEvent::Exit => {
                        event_loop.exit();
                    }
                }
            }
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
