use utils::{ApplicationRunner, HasLoggerHandle};
use thiserror::Error;
use tracing_subscriber::EnvFilter;
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

struct EmptyType {
    handle: (),
}

impl HasLoggerHandle for EmptyType {
    type Handle = ();

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
    type AppLoggerHandle = EmptyType;

    fn run(&self, cmd_args: Self::CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }

    fn configure_logging(&self) -> Self::AppLoggerHandle {
        let my_system_time_formatter = MySystemTimeFormatter;

        tracing_subscriber::fmt()
            .with_writer(make_file_writer_for_logging)
            .with_ansi(false)
            .with_env_filter(EnvFilter::from_default_env())
            .with_timer(my_system_time_formatter)
            .init();

        // "Sets the subscriber as the default for the duration of the lifetime of the returned `DefaultGuard`"
        // let guard = tracing::subscriber::set_default(subscriber);

        // TracingLoggerHandle {
        //     handle: guard,
        // }
        EmptyType {
            handle: (),
        }
    }
}

use std::fmt;
use tracing_subscriber::fmt::time::FormatTime;

struct MySystemTimeFormatter;

impl FormatTime for MySystemTimeFormatter {
    fn format_time(&self, w: &mut dyn fmt::Write) -> fmt::Result {
        write!(
            w,
            "{}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.6f")
        )
    }
}
