use insta::assert_snapshot;
use rust_telemetry::{
    clear_output, clear_timestamp, debug, error, info, set_output, set_timestamp, warn,
    MemoryWriter,
};

/// Run a block with a memory writer and fixed timestamp, then assert snapshot
macro_rules! capture {
    ($($body:tt)*) => {{
        let buffer = MemoryWriter::new();
        set_output(buffer.writer());
        set_timestamp("00:00:00.000");
        $($body)*
        assert_snapshot!(buffer.contents());
        clear_timestamp();
        clear_output();
    }};
}

#[test]
fn test_debug_output() {
    capture! {
        debug("hello world");
    }
}

#[test]
fn test_info_output() {
    capture! {
        info("server started");
    }
}

#[test]
fn test_warn_output() {
    capture! {
        warn("memory low");
    }
}

#[test]
fn test_error_output() {
    capture! {
        error("connection failed");
    }
}

#[test]
fn test_multiple_logs() {
    capture! {
        debug("first");
        info("second");
        warn("third");
        error("fourth");
    }
}
