use clap::{AppSettings, Clap, IntoApp};
use std::fmt::Debug;

#[cfg(feature = "simple_logging")]
use flexi_logger;

#[derive(Debug)]
enum RunStatus {
    OK = 0,
    Error = 1,
}

pub trait ApplicationRunner {
    type Error: std::error::Error;
    type CmdArgs: IntoApp + Clap + Debug;

    #[cfg(not(feature = "simple_logging"))]
    type AppLogger: Logger;

    fn main(&self) -> i32 {

        #[cfg(feature = "simple_logging")] {
            flexi_logger::Logger::try_with_env().unwrap().log_to_stdout().start().unwrap();
            trace!("Default logger initialized");
        }

        #[cfg(not(feature = "simple_logging"))] {
            Self::AppLogger::initialize_logger().unwrap();
            trace!("User's logger initialized");
        }

        let cmd_args = Self::CmdArgs::parse();
        trace!("Parsed command line arguments - {:?}", cmd_args);

        // maybe with application name or with chosen subcommand
        info!("Running application...");

        let status = if let Err(err) = self.run(cmd_args) {
            self.write_app_error_message(&err.to_string());
            error!("Error - {}", &err.to_string());

            RunStatus::Error
        } else {
            RunStatus::OK
        };

        info!("Closing application with status {:?}", &status);
        std::process::exit(status as i32)
    }

    fn run(&self, cmd_args: Self::CmdArgs) -> Result<(), Self::Error>;

    fn should_write_app_error_message_with_colors(&self) -> bool {
        Self::CmdArgs::into_app().is_set(AppSettings::ColoredHelp)
    }

    fn write_app_error_message(&self, error_message: &String) {
        use crate::write_colored_error_message;

        if self.should_write_app_error_message_with_colors() {
            write_colored_error_message(error_message).unwrap();
        } else {
            eprintln!("{}", error_message);
        }
    }
}

pub trait Logger {
    type InnerLogger;
    type LoggerError: Debug;

    fn initialize_logger() -> Result<Self::InnerLogger, Self::LoggerError>;
}

#[cfg(feature = "simple_logging")]
struct DefaultLogger;

#[cfg(feature = "simple_logging")]
impl Logger for DefaultLogger {
    type LoggerError = flexi_logger::FlexiLoggerError;
    type InnerLogger = flexi_logger::Logger;

    fn initialize_logger() -> Result<Self::InnerLogger,
        Self::LoggerError> {
        flexi_logger::Logger::try_with_env()
    }
}
