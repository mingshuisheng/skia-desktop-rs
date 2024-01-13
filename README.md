## Skia Desktop Development
a temporary gui lib for myself

Example
```rust
use skia_safe::{Canvas, Color};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::window::WindowBuilder;
use skia_desktop::application::launch;
use skia_desktop::context::window_context::WindowContext;
use skia_desktop::event_handler::EventHandler;
use skia_desktop::ui::UI;

pub struct MyUI {
    color: Color,
}

impl MyUI {
    fn new() -> Self {
        MyUI {
            color: Color::YELLOW
        }
    }
}

impl UI for MyUI {
    fn draw(&mut self, canvas: &Canvas) {
        canvas.clear(self.color);
    }

    fn handle_event(&mut self, event: WindowEvent, event_context: &mut WindowContext) {
        match event {
            WindowEvent::MouseInput { button, state, .. } => {
                if button == MouseButton::Left && state == ElementState::Pressed {
                    self.color = if self.color == Color::YELLOW {
                        Color::BLUE
                    } else {
                        Color::YELLOW
                    };
                    event_context.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::default();

    event_handler.add_init_handler(|context| {
        let window_builder = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(256.0, 256.0));
        let window = context.new_window(window_builder, MyUI::new()).unwrap();
        window.request_redraw();
    });

    launch(event_handler);
}
```