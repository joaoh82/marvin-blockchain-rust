use slog::*;
use std::sync::Mutex;

pub fn make_json_logger() -> Logger {
    // let term_decorator = slog_term::TermDecorator::new().build();
    // let term_drain = slog_term::FullFormat::new(term_decorator).build().fuse();
    // let term_drain = Mutex::new(term_drain).fuse();

    let json_drain = slog_json::Json::default(std::io::stdout()).fuse();
    let json_drain = Mutex::new(json_drain).fuse();

    // let logger = Logger::root(slog::Duplicate(term_drain, json_drain).fuse(), o!());
    let logger = Logger::root(json_drain.fuse(), o!());

    // info!(logger, "info json log"; o!("key" => "value", "key2" => "value2"));
    // warn!(logger, "warn json log"; "key" => "value");
    // debug!(logger, "debug json log"; "key" => "value");

    logger
}