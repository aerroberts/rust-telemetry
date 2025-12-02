mod config;
mod loggers;

pub use config::clear_output;
pub use config::set_output;
pub use loggers::debug;
pub use loggers::error;
pub use loggers::info;
pub use loggers::warn;
