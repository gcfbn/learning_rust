/// You can run this program with the following command e.g.:
///
/// RUST_LOG=error,use_default_logging=warn cargo run --example use_default_logging --features default_logging
///
mod cmd_args;

use cmd_args::CmdArgs;
use log::warn;
use thiserror::Error;
use utils::{ApplicationRunner, DefaultAppLoggerHandle};

#[derive(Debug, Error)]
#[error("Application error !")]
struct AppError;

struct App;

fn main() {
    App.main();
}

impl ApplicationRunner for App {
    type AppLoggerHandle = DefaultAppLoggerHandle;
    type CmdArgs = CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: CmdArgs) -> Result<(), Self::Error> {
        warn!("this method will raise an error");

        Err(AppError)
    }
}
