mod config;
mod loggers;
mod utils;
pub mod writers;

pub use config::clear_output;
pub use config::clear_timestamp;
pub use config::set_output;
pub use config::set_timestamp;
pub use loggers::debug;
pub use loggers::error;
pub use loggers::info;
pub use loggers::warn;
pub use writers::{FileWriter, MemoryWriter, StdoutWriter};
