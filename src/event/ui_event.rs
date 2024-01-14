use winit::event::WindowEvent;
use crate::application::TimerId;

pub enum UIEvent {
    TimerOut(TimerId),
    Interval(TimerId),
    WindowCreate,
    Other(WindowEvent)
}