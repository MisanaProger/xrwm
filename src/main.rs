use config::*;
use window_manager::*;

pub mod config;
pub mod screen;
pub mod window;
pub mod window_manager;

fn main() {
    let config = match Config::load() {
        Ok(config) => config,
        Err(error) => {
            print_error(error);
            return;
        }
    };

    let window_manager = WindowManager::new(config);
    if let Err(error) = window_manager.run() {
        print_runtime_error(error)
    }
}
