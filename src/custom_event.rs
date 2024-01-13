use winit::window::WindowId;

#[derive(Debug, Clone, Copy)]
pub enum CustomEvent{
    CloseWindow(WindowId),
    Exit
}