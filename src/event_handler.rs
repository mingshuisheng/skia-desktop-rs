use crate::controller::Controller;

pub struct EventHandler {
    handle_init: Option<Box<dyn Fn(Controller)>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            handle_init: None
        }
    }

    pub fn add_init_handler(&mut self, handle_init: impl Fn(Controller) + 'static) {
        self.handle_init = Some(
            Box::new(handle_init)
        )
    }

    pub(crate) fn on_init(&self, controller: Controller) {
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
