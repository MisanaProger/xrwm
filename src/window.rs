use std::sync::Arc;

use xcb::x::{ConfigWindow, ConfigWindowMask};

pub struct Window {
    window: xcb::x::Window,
    connection: Arc<xcb::Connection>,
}

pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

impl Window {
    pub fn new(window: xcb::x::Window, connection: Arc<xcb::Connection>) -> Window {
        Window { window, connection }
    }

    pub fn window_size(&self) -> Result<WindowSize, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetWindowAttributes {
            window: self.window.clone(),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        responce.
            }

    pub fn set_window_size(&self, window_size: WindowSize) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window.clone(),
            value_list: &[
                ConfigWindow::Height(window_size.height),
                ConfigWindow::Width(window_size.width),
            ],
        });
        self.connection.flush()
    }
}
