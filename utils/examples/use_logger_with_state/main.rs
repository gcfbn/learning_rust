/// You can run this program with the following command e.g.:
///
/// RUST_LOG=error,use_logger_with_state=warn cargo run --example use_logger_with_state --features app_logger_has_state
///

use std::fmt::Debug;
use thiserror::Error;
use tracing::warn;
use tracing_subscriber::{EnvFilter, subscribe::CollectExt, fmt::Subscriber, util::SubscriberInitExt};
use utils::{ApplicationRunner, HasLoggerHandle};

mod cmd_args;

#[derive(Debug, Error)]
#[error("Application error !")]
struct AppError;

struct App;

struct EmptyHandle {
    handle: (),
}

impl HasLoggerHandle for EmptyHandle {
    type Handle = ();

    fn handle(&self) -> &Self::Handle {
        &self.handle
    }
}

fn main() {
    App.main();
}

fn make_file_writer_for_logging() -> impl std::io::Write {
    tracing_appender::rolling::minutely("./.logs", "use_logger_with_state")
}

impl ApplicationRunner for App {
    type AppLoggerHandle = EmptyHandle;
    type CmdArgs = cmd_args::CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }

    fn configure_logging(&self) -> Self::AppLoggerHandle {

        let file_subscriber = Subscriber::new()
            .with_ansi(false)
            .with_writer(make_file_writer_for_logging)
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

        EmptyHandle { handle: () }
    }
}

use tracing_subscriber::fmt::time::FormatTime;

struct MySystemTimeFormatter;

impl FormatTime for MySystemTimeFormatter {
    fn format_time(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(w, "{:>5}", chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.6f]"))
    }
}
