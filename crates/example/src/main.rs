fn main() {
    // Initialize with Trace level to see all logs
    log::init_with_level(log::Level::Trace);

    println!("=== rust-telemetry Example ===\n");

    // Demonstrate all log levels
    log::trace!("This is a trace message - very detailed debugging");
    log::debug!("This is a debug message - debugging info");
    log::info!("This is an info message - general information");
    log::warn!("This is a warning message - something might be wrong");
    log::error!("This is an error message - something went wrong");

    println!();

    // Demonstrate formatted messages
    let user = "alice";
    let count = 42;
    log::info!("User {} has {} items in cart", user, count);

    // Demonstrate changing log level at runtime
    println!("\n--- Changing log level to Warn ---\n");
    log::set_max_level(log::Level::Warn);

    log::trace!("This trace won't show");
    log::debug!("This debug won't show");
    log::info!("This info won't show");
    log::warn!("This warning will show");
    log::error!("This error will show");

    println!("\n=== Example Complete ===");
}
