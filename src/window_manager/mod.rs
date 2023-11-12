use std::{cell::RefCell, sync::Arc, thread};

use crate::config::Config;
use xcb::{Connection, Event, UnknownEvent};

use self::tiling::TilingCompositor;

mod event_handler;
mod floating;
mod monocle;
mod tiling;

pub type RuntimeError = xcb::Error;

pub trait Compositor: Send + Sync {
    fn locate_windows(&self);
}

pub fn print_runtime_error(error: RuntimeError) {
    todo!()
}

pub struct WindowManager {
    config: Arc<Config>,
    x_server_connecton: Arc<Connection>,
    compositor: Arc<RefCell<Box<dyn Compositor>>>,
}

unsafe impl Sync for WindowManager {}
unsafe impl Send for WindowManager {}

impl WindowManager {
    pub fn new(config: Config) -> WindowManager {
        let connection = Connection::connect(None).unwrap().0;
        let connection = Arc::new(connection);
        let config = Arc::new(config);
        WindowManager {
            config: config.clone(),
            x_server_connecton: connection.clone(),
            compositor: Arc::new(RefCell::new(Box::new(TilingCompositor::new(
                config.tiling_config(),
                connection,
            )))),
        }
    }

    pub fn run(self) -> Result<(), RuntimeError> {
        let window_manager = Arc::new(self);

        loop {
            let event = window_manager.x_server_connecton.wait_for_event()?;
            let window_manager = window_manager.clone();
            thread::spawn(move || window_manager.clone().handle_event(event));
        }
    }

    pub fn handle_event(&self, event: Event) {
        match event {
            Event::X(_) => todo!(),
            Event::Shape(_) => todo!(),
            Event::XFixes(_) => todo!(),
            Event::Unknown(event) => self.handle_unknown_event(event),
        }
    }

    pub fn handle_unknown_event(&self, event: UnknownEvent) {
        todo!()
    }

    pub fn handle_shape_event(&self, event: xcb::shape::Event) {
        match event {
            xcb::shape::Event::Notify(_) => todo!(),
        }
    }

    pub fn handle_fix_event(&self, event: xcb::xfixes::Event) {
        match event {
            xcb::xfixes::Event::SelectionNotify(_) => todo!(),
            xcb::xfixes::Event::CursorNotify(_) => todo!(),
        }
    }

    pub fn handle_x_event(&self, event: xcb::x::Event) {
        match event {
            xcb::x::Event::KeyPress(_) => todo!(),
            xcb::x::Event::KeyRelease(_) => todo!(),
            xcb::x::Event::ButtonPress(_) => todo!(),
            xcb::x::Event::ButtonRelease(_) => todo!(),
            xcb::x::Event::MotionNotify(_) => todo!(),
            xcb::x::Event::EnterNotify(_) => todo!(),
            xcb::x::Event::LeaveNotify(_) => todo!(),
            xcb::x::Event::FocusIn(_) => todo!(),
            xcb::x::Event::FocusOut(_) => todo!(),
            xcb::x::Event::KeymapNotify(_) => todo!(),
            xcb::x::Event::Expose(_) => todo!(),
            xcb::x::Event::GraphicsExposure(_) => todo!(),
            xcb::x::Event::NoExposure(_) => todo!(),
            xcb::x::Event::VisibilityNotify(_) => todo!(),
            xcb::x::Event::CreateNotify(_) => todo!(),
            xcb::x::Event::DestroyNotify(_) => todo!(),
            xcb::x::Event::UnmapNotify(_) => todo!(),
            xcb::x::Event::MapNotify(_) => todo!(),
            xcb::x::Event::MapRequest(_) => todo!(),
            xcb::x::Event::ReparentNotify(_) => todo!(),
            xcb::x::Event::ConfigureNotify(_) => todo!(),
            xcb::x::Event::ConfigureRequest(_) => todo!(),
            xcb::x::Event::GravityNotify(_) => todo!(),
            xcb::x::Event::ResizeRequest(_) => todo!(),
            xcb::x::Event::CirculateNotify(_) => todo!(),
            xcb::x::Event::CirculateRequest(_) => todo!(),
            xcb::x::Event::PropertyNotify(_) => todo!(),
            xcb::x::Event::SelectionClear(_) => todo!(),
            xcb::x::Event::SelectionRequest(_) => todo!(),
            xcb::x::Event::SelectionNotify(_) => todo!(),
            xcb::x::Event::ColormapNotify(_) => todo!(),
            xcb::x::Event::ClientMessage(_) => todo!(),
            xcb::x::Event::MappingNotify(_) => todo!(),
        }
    }
}
