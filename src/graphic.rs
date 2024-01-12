use glutin::config::Config;

use raw_window_handle::RawWindowHandle;
use skia_safe::Canvas;
use winit::dpi::PhysicalSize;

use crate::glutin_graphic::GlutinGraphic;
use crate::skia::SkiaGLGraphic;

pub struct Graphic {
    skia_graphic: SkiaGLGraphic,
    glutin_graphic: GlutinGraphic,
}

impl Graphic {
    pub fn new(size: PhysicalSize<u32>, raw_window_handle: RawWindowHandle, gl_config: Config) -> Self {
        // Must be initialized first glutin
        let glutin_graphic = GlutinGraphic::new(size.into(), raw_window_handle, gl_config.clone());
        let skia_graphic = SkiaGLGraphic::new(size.into(), gl_config);
        Graphic {
            skia_graphic,
            glutin_graphic
        }
    }

    pub fn draw(&mut self, draw_fn: impl FnOnce(&Canvas)) {
        self.skia_graphic.draw(draw_fn);
    }

    pub fn submit(&mut self) {
        self.skia_graphic.submit();
        self.glutin_graphic.submit();
    }

    pub fn on_resize(&mut self, size: PhysicalSize<u32>) {
        self.skia_graphic.on_resize(size.into());
        self.glutin_graphic.on_size(size.into())
    }
}