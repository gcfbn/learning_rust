use cmd_args::CmdArgs;
use log::info;
use thiserror::Error;
use utils::ApplicationRunner;

mod cmd_args;

fn main() {
    App.main();
}

struct App;

impl ApplicationRunner for App {
    type CmdArgs = CmdArgs;
    type Error = AppError;

    fn run(&self, _cmd_args: CmdArgs) -> Result<(), Self::Error> {
        println!("hello");
        info!("info -> hello");
        Ok(())
    }
}

#[derive(Debug, Error)]
#[error("App error")]
struct AppError;