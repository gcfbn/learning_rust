use utils::{ApplicationRunner, HasLoggerHandle};
use thiserror::Error;
use tracing_subscriber::FmtSubscriber;
use std::fmt::Debug;
use std::io::Stdout;
use tracing_subscriber::fmt::format::{Pretty, Format};
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::time::SystemTime;

use tracing::warn;

mod cmd_args;

#[derive(Debug, Error)]
#[error("Application error !")]
struct AppError;

struct App;

struct TracingLoggerHandle {
    handle: FmtSubscriber<Pretty, Format<Pretty, SystemTime>, LevelFilter, fn() -> Stdout>,
}

impl HasLoggerHandle for TracingLoggerHandle {
    type Handle = FmtSubscriber<Pretty, Format<Pretty, SystemTime>, LevelFilter, fn() -> Stdout>;

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

        tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize global subscriber");

        // this should be rewritten
        let subscriber2 = FmtSubscriber::builder()
            .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
            .pretty()
            .finish();

        TracingLoggerHandle {
            handle: subscriber2
        }
    }
}