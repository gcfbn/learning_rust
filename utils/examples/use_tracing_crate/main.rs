/// You can run this program with the following command e.g.:
///
/// RUST_LOG=error,use_tracing_crate=warn cargo run --example use_tracing_crate --features tracing_example
///
mod cmd_args;

use std::fmt::Debug;
use thiserror::Error;
use tracing::{warn, error};
use tracing_subscriber::{fmt::{Subscriber, Collector}, subscribe::CollectExt, util::SubscriberInitExt, EnvFilter};
use utils::ApplicationRunner;
use opentelemetry::{global, trace::Tracer};

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
    fn configure_opentelemetry(&self) -> OpenTelemetrySubscriber<tracing_subscriber::Registry, opentelemetry::sdk::trace::Tracer> {
        global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());

        let tracer = opentelemetry_jaeger::new_pipeline()
            .with_service_name("application_runner")
            .install_simple()
            .unwrap();

        // ERROR: the trait bound `opentelemetry::sdk::trace::Tracer: opentelemetry::trace::tracer::Tracer` is not satisfied
        // ERROR: the trait bound `opentelemetry::sdk::trace::Tracer: PreSampledTracer` is not satisfied
        let telemetry: OpenTelemetrySubscriber<tracing_subscriber::Registry, opentelemetry::sdk::trace::Tracer> = tracing_opentelemetry::OpenTelemetrySubscriber::default().with_tracer(tracer);

        // opentelemetry_jaeger::new_pipeline().install_simple().unwrap()
        telemetry
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
        let tracer = self.configure_opentelemetry();
        // let registry = tracing_subscriber::Registry::default().with(tracer);

        let file_subscriber = Subscriber::new()
            .with_ansi(false)
            .with_writer(|| tracing_appender::rolling::minutely("./.logs", "use_logger_with_state"))
            .with_timer(MySystemTimeFormatter);

        let stdout_subscriber = Subscriber::new()
            .with_writer(std::io::stdout)
            .with_timer(MySystemTimeFormatter);

        // see: https://github.com/tokio-rs/tracing/blob/master/examples/examples/fmt-multiple-writers.rs
        let registry = tracing_subscriber::registry()
            .with(EnvFilter::from_default_env())
            .with(file_subscriber)
            .with(stdout_subscriber)
            .init();
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
