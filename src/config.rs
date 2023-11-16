use std::{
    fs::{create_dir, File},
    io::Read,
};

use toml::de::Error;

pub struct Config {
    tiling_config: TilingConfig,
}

#[derive(Clone)]
pub struct TilingConfig {
    gaps_config: Option<GapsConfig>,
}

#[derive(Clone)]
pub struct WindowConfig {
    border_size: u16,
    border_radius: u16,
    open_in_center_on_floating_mode: bool,
}

#[derive(Clone)]
pub struct GapsConfig {
    inner_size: u16,
    outer_size: u16,
}

pub enum LoadingConfigError {
    TomlParsingError(Error),
}

pub fn print_error(error: LoadingConfigError) {
    todo!()
}

impl Config {
    pub fn load() -> Result<Config, LoadingConfigError> {
        let home_dir = env!("HOME");
        let path_to_config = format!("{}/.config/xrwm/Config.toml", home_dir);
        let mut file = match File::open(path_to_config) {
            Ok(file) => file,
            Err(err) => {
                create_dir(".config/xrwm");
                File::create(path_to_config).expect("can't create file")
            }
        };
        let mut config_string = String::new();
        file.read_to_string(&mut config_string);
        match toml::from_str(&config_string) {
            Ok(config) => Ok(config),
            Err(error) => Err(LoadingConfigError::TomlParsingError(error)),
        }
    }

    pub fn tiling_config(&self) -> TilingConfig {
        self.tiling_config.clone()
    }
}

impl TilingConfig {
    pub fn gaps_config(&self) -> Option<GapsConfig> {
        self.gaps_config.clone()
    }
}
