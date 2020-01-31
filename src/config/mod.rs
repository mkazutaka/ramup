pub use self::config::Config;
pub use self::ram_config::RAM;

#[allow(clippy::module_inception)]
mod config;
mod ram_config;
