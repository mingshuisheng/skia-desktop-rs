use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::event::{Event, StartCause};
use winit::event_loop::{ControlFlow, EventLoop, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget};
use winit::window::WindowId;
use crate::context::context::Context;
use crate::custom_event::CustomEvent;
use crate::event_handler::EventHandler;
use crate::window::Window;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TimerId(u32);

impl TimerId {
    pub fn next(&self) -> Self {
        Self(self.0 + 1)
    }
}

pub struct Application {
    event_loop_proxy: EventLoopProxy<CustomEvent>,
    window_map: HashMap<WindowId, Window>,
    timer_map: HashMap<TimerId, (Instant, Box<dyn FnOnce(TimerId, &mut Application, &EventLoopWindowTarget<CustomEvent>)>)>,
    timer_id: TimerId,
    interval_map: HashMap<TimerId, (Instant, Duration, Box<dyn Fn(TimerId, &mut Application, &EventLoopWindowTarget<CustomEvent>)>)>,
    interval_id: TimerId,
}

impl Application {
    pub fn new(event_loop_proxy: EventLoopProxy<CustomEvent>) -> Self {
        Self {
            event_loop_proxy,
            window_map: HashMap::new(),
            timer_map: HashMap::new(),
            timer_id: TimerId::default(),
            interval_map: HashMap::new(),
            interval_id: TimerId::default(),
        }
    }

    pub fn add_timer(&mut self, time: Duration, f: Box<dyn FnOnce(TimerId, &mut Application, &EventLoopWindowTarget<CustomEvent>)>) -> TimerId {
        let id = self.timer_id.next();
        let now = Instant::now();
        self.timer_map.insert(id, (now + time, f));
        id
    }

    pub fn remove_timer(&mut self, timer_id: TimerId) {
        self.timer_map.remove(&timer_id);
    }

    pub fn check_and_run_timer(&mut self, event_loop: &EventLoopWindowTarget<CustomEvent>) {
        if self.timer_map.is_empty() {
            return;
        }
        let now = Instant::now();
        let ids: Vec<_> = self.timer_map.iter().filter(|(_, data)| now >= data.0).map(|(id, _)| id.clone()).collect();
        for id in ids {
            let (_, f) = self.timer_map.remove(&id).unwrap();
            f(id, self, event_loop);
        }
    }

    pub fn add_interval(&mut self, time: Duration, f: Box<dyn Fn(TimerId, &mut Application, &EventLoopWindowTarget<CustomEvent>)>) -> TimerId {
        let id = self.interval_id.next();
        let now = Instant::now();
        self.interval_map.insert(id, (now, time, f));
        id
    }

    pub fn remove_interval(&mut self, timer_id: TimerId) {
        self.timer_map.remove(&timer_id);
    }

    pub fn check_and_run_interval(&mut self, event_loop: &EventLoopWindowTarget<CustomEvent>) {
        if self.interval_map.is_empty() {
            return;
        }
        let now = Instant::now();
        let ids: Vec<_> = self.interval_map.iter().filter(|(_, &(last, time, _))| now >= last + time).map(|(id, _)| id.clone()).collect();
        for id in ids {
            let (_, time, f) = self.interval_map.remove(&id).unwrap();
            f(id, self, event_loop);
            self.interval_map.insert(id, (now, time, f));
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
    pub fn request_exit(&mut self) {
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
    event_loop.set_control_flow(ControlFlow::Poll);
    let event_loop_proxy = event_loop.create_proxy();
    let mut application = Application::new(event_loop_proxy);

    event_loop.run(move |event, event_loop| {
        application.check_and_run_timer(event_loop);
        application.check_and_run_interval(event_loop);
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

            Event::LoopExiting => {}
            Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {}
            Event::NewEvents(StartCause::WaitCancelled { .. }) => {}
            Event::NewEvents(StartCause::Poll) => {}
            _ => {}
        }
    }).unwrap();
}
