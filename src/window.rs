use std::{cell::RefCell, sync::Arc};

use xcb::x::ConfigWindow;

pub struct Window {
    window: xcb::x::Window,
    connection: Arc<xcb::Connection>,
    current_tag: RefCell<u32>,
    awailable_tags: TagPermitions,
}

#[derive(Clone)]
pub enum TagPermitions {
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

impl Window {
    pub fn new(
        window: xcb::x::Window,
        connection: Arc<xcb::Connection>,
        current_tag: u32,
        awailable_tags: TagPermitions,
    ) -> Window {
        let window = Window {
            window,
            connection,
            current_tag: RefCell::new(current_tag),
            awailable_tags,
        };
        window.move_to(current_tag);
        window
    }

    fn window(&self) -> xcb::x::Window {
        self.window.clone()
    }

    pub fn window_size(&self) -> Result<WindowSize, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(WindowSize {
            height: responce.height(),
            width: responce.width(),
        })
    }

    pub fn resize_window(&self, window_size: WindowSize) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window(),
            value_list: &[
                ConfigWindow::Height(window_size.height() as u32),
                ConfigWindow::Width(window_size.width() as u32),
            ],
        });
        self.connection.flush()
    }

    pub fn window_position(&self) -> Result<WindowPosition, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(WindowPosition {
            x: responce.x(),
            y: responce.y(),
        })
    }

    pub fn change_window_position(
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

    pub fn border_size(&self) -> Result<u16, xcb::Error> {
        let cookie = self.connection.send_request(&xcb::x::GetGeometry {
            drawable: xcb::x::Drawable::Window(self.window()),
        });
        let responce = self.connection.wait_for_reply(cookie)?;
        Ok(responce.border_width())
    }

    pub fn change_border_size(&self, border_size: u16) -> Result<(), xcb::ConnError> {
        self.connection.send_request(&xcb::x::ConfigureWindow {
            window: self.window(),
            value_list: &[ConfigWindow::BorderWidth(border_size as u32)],
        });
        self.connection.flush()
    }

    pub fn move_to(&self, tag: u32) {
        match self.awailable_tags.clone() {
            TagPermitions::All => *(self.current_tag).borrow_mut() = tag,
            TagPermitions::Only(awailable_tags) => {
                if awailable_tags.contains(&tag) || awailable_tags.is_empty() {
                    *(self.current_tag).borrow_mut() = tag;
                    return;
                }
                *(self.current_tag).borrow_mut() = *awailable_tags.first().unwrap();
            }
        }
    }

    pub fn current_tag(&self) -> u32 {
        self.current_tag.clone().into_inner()
    }
}
