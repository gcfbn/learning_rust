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
            .pretty()
            .finish();

        let guard = tracing::subscriber::set_default(subscriber);

        TracingLoggerHandle {
            handle: guard,
        }
    }
}