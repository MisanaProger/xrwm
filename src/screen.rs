use std::sync::Arc;

use xcb::Connection;

pub struct Screen {
    connection: Arc<Connection>,
    screen: xcb::x::Screen,
}

impl Screen {
    pub fn new(connection: Connection) -> Screen {
        f
    }
}
