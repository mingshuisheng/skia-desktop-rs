use std::ffi::CString;
use std::num::NonZeroU32;
use glutin::config::Config;
use glutin::context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};
use glutin::surface::Surface;
use raw_window_handle::RawWindowHandle;

pub struct GlutinGraphic {
    gl_surface: Surface<WindowSurface>,
    gl_context: PossiblyCurrentContext,
}

impl GlutinGraphic {
    pub fn new((width, height): (u32, u32), raw_window_handle: RawWindowHandle, gl_config: Config,) -> Self{
        
        let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_window_handle));

        let not_current_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)
                .unwrap_or_else(|_| {
                    gl_config
                        .display()
                        .create_context(&gl_config, &fallback_context_attributes)
                        .expect("failed to create context")
                })
        };


        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );


        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .expect("Could not create gl window surface")
        };

        let gl_context = not_current_gl_context
            .make_current(&gl_surface)
            .expect("Could not make GL context current when setting up skia renderer");

        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(CString::new(s).unwrap().as_c_str())
        });

        GlutinGraphic{
            gl_surface,
            gl_context
        }
    }
    
    pub fn submit(&self){
        self.gl_surface.swap_buffers(&self.gl_context).unwrap();
    }
    
    pub fn on_size(&self, (width, height): (u32, u32)){
        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        ); 
    }
}