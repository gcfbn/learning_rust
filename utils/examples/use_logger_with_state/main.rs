use utils::{ApplicationRunner, HasLoggerHandle};
use thiserror::Error;
use tracing_subscriber::FmtSubscriber;
use std::fmt::Debug;
use tracing::warn;
use tracing::dispatcher::DefaultGuard;

mod cmd_args;

#[derive(Debug, Error)]
#[error("Application error !")]
struct AppError;

struct App;

struct TracingLoggerHandle {
    handle: DefaultGuard,
}

impl HasLoggerHandle for TracingLoggerHandle {
    type Handle = DefaultGuard;

    fn handle(&self) -> &Self::Handle {
        &self.handle
    }
}

fn main() {
    App.main();
}

fn make_file_writer_for_logging() -> impl std::io::Write {
    // log files have extension like `.2021-08-25-19-34` and aren't displayed properly, at least on my device
    tracing_appender::rolling::minutely("./.logs", "use_logger_with_state")
}

impl ApplicationRunner for App {
    type Error = AppError;
    type CmdArgs = cmd_args::CmdArgs;
    type AppLoggerHandle = TracingLoggerHandle;

    fn run(&self, cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }

    fn configure_logging(&self) -> Self::AppLoggerHandle {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
            .with_writer(make_file_writer_for_logging)
            .finish();

        // "Sets the subscriber as the default for the duration of the lifetime of the returned `DefaultGuard`"
        let guard = tracing::subscriber::set_default(subscriber);

        TracingLoggerHandle {
            handle: guard,
        }
    }
}