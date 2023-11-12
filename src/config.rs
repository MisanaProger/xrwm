pub struct Config {
    tiling_config: TilingConfig,
}

pub struct TilingConfig {
    gaps_config: Option<GapsConfig>,
}

pub struct GapsConfig {}

pub enum LoadingConfigError {}

pub fn print_error(error: LoadingConfigError) {
    todo!()
}

impl Config {
    pub fn load() -> Result<Config, LoadingConfigError> {
        todo!()
    }

    pub fn tiling_config(&self) -> TilingConfig {
        self.tiling_config
    }
}

impl TilingConfig {
    pub fn gaps_config(&self) -> GapsConfig {
        self.gaps_config()
    }
}
