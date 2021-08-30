/// You can run this program with the following command e.g.:
///
/// RUST_LOG=error,use_tracing_crate=warn cargo run --example use_tracing_crate --features tracing_example
///
mod cmd_args;

use std::fmt::Debug;
use thiserror::Error;
use tracing::warn;
use tracing_subscriber::{fmt::Subscriber, subscribe::CollectExt, util::SubscriberInitExt, EnvFilter};
use utils::ApplicationRunner;

// -----------------------------------------------------------------------------

fn main() {
    App.main();
}

// -----------------------------------------------------------------------------

#[derive(Debug, Error)]
#[error("Application error !")]
struct AppError;

// -----------------------------------------------------------------------------

struct App;

impl ApplicationRunner for App {
    type CmdArgs = cmd_args::CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }

    fn configure_logging(&self) {
        let file_subscriber = Subscriber::new()
            .with_ansi(false)
            .with_writer(|| tracing_appender::rolling::minutely("./.logs", "use_logger_with_state"))
            .with_timer(MySystemTimeFormatter);

        let stdout_subscriber = Subscriber::new()
            .with_writer(std::io::stdout)
            .with_timer(MySystemTimeFormatter);

        // see: https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
        tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(file_subscriber)
            .with(stdout_subscriber)
            .init();
    }
}

// -----------------------------------------------------------------------------

use tracing_subscriber::fmt::time::FormatTime;

struct MySystemTimeFormatter;

impl FormatTime for MySystemTimeFormatter {
    fn format_time(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"))
    }
}

// -----------------------------------------------------------------------------
