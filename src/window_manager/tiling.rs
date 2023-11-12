use super::Compositor;
use crate::config::TilingConfig;
use std::sync::Arc;
use xcb::Connection;

pub struct TilingCompositor {
    tiling_config: TilingConfig,
    x_connection: Arc<Connection>,
}

impl TilingCompositor {
    pub fn new(tiling_config: TilingConfig, x_connection: Arc<Connection>) -> TilingCompositor {
        TilingCompositor {
            tiling_config,
            x_connection,
        }
    }
}

impl Compositor for TilingCompositor {
    fn locate_windows(&self) {
        todo!()
    }
}
