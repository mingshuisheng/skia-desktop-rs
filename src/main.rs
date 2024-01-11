use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod graphic;
mod window;
mod skia;
mod glutin_graphic;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window_builder = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0));

    let mut window = window::Window::new(window_builder, &event_loop);

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. }  => match event {
                WindowEvent::Resized(physical_size) => {
                    window.on_resize(physical_size);
                }
                WindowEvent::CloseRequested => elwt.exit(),
                WindowEvent::RedrawRequested => {
                    window.draw();
                }
                _ => (),
            },
            Event::AboutToWait => {
                window.request_redraw();
            }

            _ => (),
        }
    }).unwrap();
}