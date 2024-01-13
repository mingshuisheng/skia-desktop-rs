use crate::context::context::Context;


pub struct EventHandler {
    handle_init: Option<Box<dyn Fn(&mut Context)>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            handle_init: None
        }
    }

    pub fn add_init_handler(&mut self, handle_init: impl Fn(&mut Context) + 'static) {
        self.handle_init = Some(
            Box::new(handle_init)
        )
    }

    pub(crate) fn on_init(&self, event_context: &mut Context) {
        if let Some(handler) = &self.handle_init {
            handler(event_context);
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
