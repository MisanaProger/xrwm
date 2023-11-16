use std::{cell::RefCell, sync::Arc};

use xcb::x::ConfigWindow;

pub struct XWindow {
    window: xcb::x::Window,
    connection: Arc<xcb::Connection>,
    current_tag: RefCell<u32>,
    allowed_tags: TagRules,
}

#[derive(Clone)]
pub enum TagRules {
    All,
    Only(Vec<u32>),
}

#[derive(Clone)]
pub struct WindowPosition {
    x: i16,
    y: i16,
}

impl WindowPosition {
    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }
}

#[derive(Clone)]
pub struct WindowSize {
    height: u16,
    width: u16,
}

impl WindowSize {
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn width(&self) -> u16 {
        self.width
    }
}

trait Window {
    //Window size
    fn window_size(&self) -> Result<WindowSize, xcb::Error>;
    fn resize_window(&self, window_size: WindowSize) -> Result<(), xcb::ConnError>;

    //Window position
    fn window_position(&self) -> Result<WindowPosition, xcb::Error>;
    fn change_window_position(&self, window_position: WindowPosition)
        -> Result<(), xcb::ConnError>;

    //Border
    fn border_size(&self) -> Result<u16, xcb::Error>;
    fn change_border_size(&self, border_size: u16) -> Result<(), xcb::ConnError>;

    //Tags
    fn move_to(&self, tag: u32);
    fn current_tag(&self) -> u32;
    fn allowed_tags(&self) -> TagRules;
}

impl XWindow {
    pub fn new(
        window: xcb::x::Window,
        connection: Arc<xcb::Connection>,
        current_tag: u32,
        allowed_tags: TagRules,
    ) -> Self {
        let window = XWindow {
            window,
            connection,
            current_tag: RefCell::new(current_tag),
            allowed_tags,
        };
        window.move_to(current_tag);
        window
    }

    fn window(&self) -> xcb::x::Window {
        self.window.clone()
    }

    fn set_current_tag(&self, new_tag: u32) {
        *(self.current_tag).borrow_mut() = new_tag
    }
}

impl Window for XWindow {
    //Window size
    fn window_size(&self) -> Result<WindowSize, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(WindowSize {
            height: responce.height(),
            width: responce.width(),
        })
    }

    fn resize_window(&self, window_size: WindowSize) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window(),
            value_list: &[
                ConfigWindow::Height(window_size.height() as u32),
                ConfigWindow::Width(window_size.width() as u32),
            ],
        });
        self.connection.flush()
    }

    //Window position
    fn window_position(&self) -> Result<WindowPosition, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(WindowPosition {
            x: responce.x(),
            y: responce.y(),
        })
    }

    fn change_window_position(
        &self,
        window_position: WindowPosition,
    ) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window(),
            value_list: &[
                ConfigWindow::X(window_position.x() as i32),
                ConfigWindow::Y(window_position.y() as i32),
            ],
        });
        self.connection.flush()
    }

    //Border size
    fn border_size(&self) -> Result<u16, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(responce.border_width())
    }

    fn change_border_size(&self, border_size: u16) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window(),
            value_list: &[ConfigWindow::BorderWidth(border_size as u32)],
        });
        self.connection.flush()
    }

    fn allowed_tags(&self) -> TagRules {
        self.allowed_tags.clone()
    }

    //Tags
    fn move_to(&self, tag: u32) {
        match self.allowed_tags.clone() {
            TagRules::All => self.set_current_tag(tag),
            TagRules::Only(allowed_tags) => {
                if allowed_tags.contains(&tag) || allowed_tags.is_empty() {
                    self.set_current_tag(tag);
                    return;
                }

                self.set_current_tag(*allowed_tags.first().unwrap())
            }
        }
    }

    fn current_tag(&self) -> u32 {
        self.current_tag.clone().into_inner()
    }
}
