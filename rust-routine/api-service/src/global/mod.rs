use lazy_static::lazy_static;

use crate::config;

lazy_static! {
    pub static ref CONFIGS: config::Configs =
        config::Configs::init_config().expect("Failed to load configs");
}
