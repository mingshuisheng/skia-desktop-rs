use skia_safe::Canvas;

pub trait UI{
    fn draw(&mut self, canvas: &Canvas);
}