/// You can run this program with the following command e.g.:
///
/// RUST_LOG=error,use_tracing_crate=warn cargo run --example use_tracing_crate --features tracing_example
///
mod cmd_args;

use std::fmt::Debug;
use thiserror::Error;
use tracing::{warn, error, trace, Level, span, };
use tracing_subscriber::{fmt::Subscriber, EnvFilter, prelude::*, util::SubscriberInitExt, registry::Registry};
use utils::ApplicationRunner;
use opentelemetry::{global, sdk::trace::Tracer};

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

impl App {
    fn configure_opentelemetry(&self) -> Tracer {
        opentelemetry_jaeger::new_pipeline()
            .with_service_name("application_runner")
            .install_simple()
            .unwrap()
    }
}

impl ApplicationRunner for App {
    type CmdArgs = cmd_args::CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }

    fn configure_logging(&self) {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let file_subscriber = Subscriber::new()
            .with_ansi(false)
            .with_writer(|| tracing_appender::rolling::minutely("./.logs", "use_logger_with_state"))
            .with_timer(MySystemTimeFormatter);

        let stdout_subscriber = Subscriber::new()
            .with_writer(std::io::stdout)
            .with_timer(MySystemTimeFormatter);

        let telemetry = OpenTelemetrySubscriber::new(self.configure_opentelemetry());

        // see: https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
        Registry::default()
            .with(EnvFilter::from_default_env())
            .with(file_subscriber)
            .with(stdout_subscriber)
            .with(telemetry)
            .init();

        let test_span = span!(Level::TRACE, "test_span");
        let _enter = test_span.enter();
        trace!("entered test_span");
    }
}

// -----------------------------------------------------------------------------

use tracing_subscriber::fmt::time::FormatTime;
use tracing_opentelemetry::OpenTelemetrySubscriber;

struct MySystemTimeFormatter;

impl FormatTime for MySystemTimeFormatter {
    fn format_time(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"))
    }
}

// -----------------------------------------------------------------------------
