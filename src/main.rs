use skia_safe::{Canvas, Color};
use winit::window::WindowBuilder;
use skia_desktop_rs::application::launch;
use skia_desktop_rs::event_handler::EventHandler;
use skia_desktop_rs::ui::UI;


pub struct MyUI;

impl UI for MyUI{
    fn draw(&mut self, canvas: &Canvas) {
        canvas.clear(Color::BLUE);
    }
}

fn main() {

    let mut event_handler = EventHandler::default();

    event_handler.add_init_handler(|mut controller| {
        let window_builder = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
        let window = controller.new_window(window_builder, MyUI).unwrap();
        window.request_redraw();
    });

    launch(event_handler);
}