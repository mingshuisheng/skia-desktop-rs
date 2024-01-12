use std::ffi::CString;
use gl::types::GLint;
use glutin::config::{Config, GlConfig};
use glutin::display::{GetGlDisplay, GlDisplay};
use skia_safe::gpu::{backend_render_targets, DirectContext, gl::{Interface, Format}, SurfaceOrigin};
use skia_safe::gpu::gl::FramebufferInfo;
use skia_safe::{ColorType, gpu, Surface, Canvas};

pub struct SkiaGLGraphic {
    surface: Surface,
    gr_context: DirectContext,
    fb_info: FramebufferInfo,
    num_samples: usize,
    stencil_size: usize,
}

impl SkiaGLGraphic {
    pub fn new(size: (i32, i32), gl_config: Config) -> Self {
        let interface = Interface::new_load_with(|name| {
            if name == "eglGetCurrentDisplay" {
                return std::ptr::null();
            }
            gl_config
                .display()
                .get_proc_address(CString::new(name).unwrap().as_c_str())
        })
            .expect("Could not create interface");
        let mut gr_context = DirectContext::new_gl(Some(interface), None)
            .expect("Could not create direct context");
        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

            FramebufferInfo {
                fboid: fboid.try_into().unwrap(),
                format: Format::RGBA8.into(),
                ..Default::default()
            }
        };
        let num_samples = gl_config.num_samples() as usize;
        let stencil_size = gl_config.stencil_size() as usize;
        let surface = create_surface(size, fb_info, &mut gr_context, num_samples, stencil_size);
        SkiaGLGraphic {
            surface,
            gr_context,
            fb_info,
            num_samples,
            stencil_size,
        }
    }

    pub fn draw(&mut self, draw_fn: impl FnOnce(&Canvas)){
        let canvas = self.surface.canvas();
        draw_fn(canvas);
    }

    pub fn submit(&mut self){
        DirectContext::flush_and_submit(&mut self.gr_context);
    }

    pub fn on_resize(&mut self, size: (i32, i32)) {
        self.surface = create_surface(
            size,
            self.fb_info,
            &mut self.gr_context,
            self.num_samples,
            self.stencil_size,
        );
    }
}


fn create_surface(
    size: (i32, i32),
    fb_info: FramebufferInfo,
    gr_context: &mut skia_safe::gpu::DirectContext,
    num_samples: usize,
    stencil_size: usize,
) -> Surface {
    let backend_render_target =
        backend_render_targets::make_gl(size, num_samples, stencil_size, fb_info);

    gpu::surfaces::wrap_backend_render_target(
        gr_context,
        &backend_render_target,
        SurfaceOrigin::BottomLeft,
        ColorType::RGBA8888,
        None,
        None,
    )
        .expect("Could not create skia surface")
}