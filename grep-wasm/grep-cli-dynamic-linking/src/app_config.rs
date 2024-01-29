use crate::app_error::AppError;
use crate::args::Args;
use std::fs::File;
use std::io::Read;
use wasmtime::Config;

pub struct AppConfig {
    pub wasmtime_config: Config,
    pub filter: Vec<u8>,
    pub host_code: Vec<u8>,
    pub files: Vec<String>,
}

impl From<Args> for AppConfig {
    fn from(args: Args) -> AppConfig {
        AppConfig {
            wasmtime_config: AppConfig::creat_config(),
            filter: Self::load_filter_file_or_default(&args.filter),
            host_code: Self::load_host_code(),
            files: args.files,
        }
    }
}

impl AppConfig {
    fn creat_config() -> Config {
        let mut config = Config::default();
        config.wasm_component_model(true);
        config.async_support(true);
        config
    }

    pub fn load_filter_file_or_default(filter_file: &Option<String>) -> Vec<u8> {
        if let Ok(filter) = Self::load_filter_file(filter_file) {
            filter
        } else {
            Self::load_default_filter()
        }
    }

    fn load_filter_file(filter_file: &Option<String>) -> anyhow::Result<Vec<u8>> {
        let filter_file = filter_file.as_ref().ok_or(AppError::NoFilterIsSpecified)?;
        let mut file = File::open(filter_file)?;
        let mut buffer = vec![];
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn load_default_filter() -> Vec<u8> {
        let id = include_bytes!("./wasm/filter_id.wasm");
        id.to_vec()
    }

    fn load_host_code() -> Vec<u8> {
        let cli = include_bytes!("./wasm/grep-cli.wasm");
        cli.to_vec()
    }
}
