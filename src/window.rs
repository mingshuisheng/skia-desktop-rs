use glutin::config::{ConfigTemplateBuilder, Config as GLConfig, GlConfig};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::window::Window as WinitWindow;
use crate::graphic::Graphic;

pub struct Window{
    inner_window: WinitWindow,
    graphic: Graphic
}

impl Window {
    pub fn new<T: 'static>(wb: WindowBuilder, event_loop: &EventLoop<T>) -> Self {
        let (window, gl_config) = create_window_and_gl_config(wb, event_loop);
        let inner_window = window.expect("create winit window error");
        let graphic = Graphic::new(inner_window.inner_size(), inner_window.raw_window_handle(), gl_config);
        Window{
            inner_window,
            graphic
        }
    }

    pub fn on_resize(&mut self, size: PhysicalSize<u32>){
        self.graphic.on_resize(size)
    }

    pub fn draw(&mut self){
        self.inner_window.pre_present_notify();
        self.graphic.draw();
        self.graphic.submit();
    }

    pub fn request_redraw(&self){
        self.inner_window.request_redraw();
    }
}

fn create_window_and_gl_config<T: 'static>(wb: WindowBuilder, event_loop: &EventLoop<T>) -> (Option<WinitWindow>, GLConfig){
    let template = ConfigTemplateBuilder::new()
        .with_alpha_size(8)
        .with_transparency(true);
    let display_builder = DisplayBuilder::new().with_window_builder(Some(wb));
    display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() < accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap()
}