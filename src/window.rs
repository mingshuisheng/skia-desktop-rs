use glutin::config::{ConfigTemplateBuilder, Config as GLConfig, GlConfig};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{EventLoopWindowTarget};
use winit::window::{WindowBuilder, WindowId};
use winit::window::Window as WinitWindow;
use crate::application::Application;
use crate::context::window_context::WindowContext;
use crate::custom_event::CustomEvent;
use crate::graphic::Graphic;
use crate::ui::UI;

pub struct Window{
    inner_window: WinitWindow,
    graphic: Graphic,
    ui: Box<dyn UI>
}

impl Window {
    pub fn new<T: 'static>(wb: WindowBuilder, event_loop: &EventLoopWindowTarget<T>, ui: impl UI + 'static) -> Self {
        let (window, gl_config) = create_window_and_gl_config(wb, event_loop);
        let inner_window = window.expect("create winit window error");
        let graphic = Graphic::new(inner_window.inner_size(), inner_window.raw_window_handle(), gl_config);
        Window{
            inner_window,
            graphic,
            ui: Box::new(ui)
        }
    }
    
    pub fn id(&self) -> WindowId{
        self.inner_window.id()
    }

    pub fn on_resize(&mut self, size: PhysicalSize<u32>){
        self.graphic.on_resize(size)
    }

    pub fn draw(&mut self){
        self.inner_window.pre_present_notify();
        self.graphic.draw(|canvas|{
            self.ui.draw(canvas);
        });
        self.graphic.submit();
    }

    pub fn request_redraw(&self){
        self.inner_window.request_redraw();
    }
    
    pub fn handle_event(&mut self, event: WindowEvent, application: &mut Application, event_loop: &EventLoopWindowTarget<CustomEvent>){
        match event {
            WindowEvent::ActivationTokenDone { .. } => {}
            WindowEvent::Resized(size) => {
                self.on_resize(size);
            }
            WindowEvent::Moved(_) => {}
            WindowEvent::CloseRequested => {}
            WindowEvent::Destroyed => {}
            WindowEvent::DroppedFile(_) => {}
            WindowEvent::HoveredFile(_) => {}
            WindowEvent::HoveredFileCancelled => {}
            WindowEvent::Focused(_) => {}
            WindowEvent::KeyboardInput { .. } => {}
            WindowEvent::ModifiersChanged(_) => {}
            WindowEvent::Ime(_) => {}
            WindowEvent::CursorMoved { .. } => {}
            WindowEvent::CursorEntered { .. } => {}
            WindowEvent::CursorLeft { .. } => {}
            WindowEvent::MouseWheel { .. } => {}
            WindowEvent::MouseInput { .. } => {}
            WindowEvent::TouchpadMagnify { .. } => {}
            WindowEvent::SmartMagnify { .. } => {}
            WindowEvent::TouchpadRotate { .. } => {}
            WindowEvent::TouchpadPressure { .. } => {}
            WindowEvent::AxisMotion { .. } => {}
            WindowEvent::Touch(_) => {}
            WindowEvent::ScaleFactorChanged { .. } => {}
            WindowEvent::ThemeChanged(_) => {}
            WindowEvent::Occluded(_) => {}
            WindowEvent::RedrawRequested => {
                self.draw();
            }
        }
        self.ui.handle_event(event, &mut WindowContext::new(application, event_loop, &mut self.inner_window));
    }

    pub(crate) fn on_close(&self) {
    }
}

fn create_window_and_gl_config<T: 'static>(wb: WindowBuilder, event_loop: &EventLoopWindowTarget<T>) -> (Option<WinitWindow>, GLConfig){
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