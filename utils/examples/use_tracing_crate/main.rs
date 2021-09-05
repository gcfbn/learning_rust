/// Please install Jaeger to see results of this example:
/// https://www.jaegertracing.io/docs/1.25/getting-started/
///
/// Run Jaeger and navigate to http://localhost:16686 to access the Jaeger UI.
///
/// Then, run the program with the following command e.g.:
/// RUST_LOG=trace&&cargo run --example use_tracing_crate --features tracing_example
///
/// Your Jaeger UI should look like similar to this:
/// https://imgur.com/a/5fZuLND
mod cmd_args;

use opentelemetry::global;
use std::fmt::Debug;
use thiserror::Error;
use tracing::{error, span, trace, warn, Level};
use tracing_subscriber::{fmt::Subscriber, prelude::*, registry::Registry, util::SubscriberInitExt, EnvFilter};
use utils::{AppLoggerHasState, ApplicationRunner};

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

pub struct AppLoggerState;

impl AppLoggerHasState for AppLoggerState {
    type State = ();

    fn new() -> Self {
        // send opentelemetry data to Jaeger
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        // write logging messages to .log file
        let file_subscriber = Subscriber::new()
            // allow non-ANSI characters
            .with_ansi(false)
            // create new file every minute
            .with_writer(|| tracing_appender::rolling::minutely("./.logs", "use_logger_with_state"))
            // use own time formatter
            .with_timer(MySystemTimeFormatter);

        // write logging messages to standard output
        let stdout_subscriber = Subscriber::new()
            .with_writer(std::io::stdout)
            .with_timer(MySystemTimeFormatter);

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("application_runner")
            .install_simple()
            .unwrap();

        let telemetry_subscriber = OpenTelemetrySubscriber::new(tracer);

        // see: https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
        Registry::default()
            .with(EnvFilter::from_default_env())
            .with(file_subscriber)
            .with(stdout_subscriber)
            .with(telemetry_subscriber)
            .init();

        // create and enter test_span, which should be visible in Jaeger UI
        let test_span = span!(Level::TRACE, "test_span");
        let _enter = test_span.enter();
        trace!("entered test_span");

        AppLoggerState
    }
}

impl Drop for AppLoggerState {
    fn drop(&mut self) {
        global::shutdown_tracer_provider(); // sending remaining spans
    }
}

impl ApplicationRunner for App {
    type AppLoggerState = AppLoggerState;
    type CmdArgs = cmd_args::CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }
}

// -----------------------------------------------------------------------------

use tracing_opentelemetry::OpenTelemetrySubscriber;
use tracing_subscriber::fmt::time::FormatTime;

struct MySystemTimeFormatter;

impl FormatTime for MySystemTimeFormatter {
    fn format_time(&self, w: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f"))
    }
}

// -----------------------------------------------------------------------------
