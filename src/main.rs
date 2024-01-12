use winit::{
    window::WindowBuilder,
};
use skia_desktop_rs::application::{EventHandler, launch};

fn main() {

    let mut event_handler = EventHandler::default();

    event_handler.set_init_handler(Some(Box::new(|mut controller| {
        let window_builder = WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));
        let window = controller.new_window(window_builder).unwrap();
        window.request_redraw();
    })));

    launch(event_handler);
}