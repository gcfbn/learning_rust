use clap::{AppSettings, Clap, IntoApp};
use std::fmt::Debug;

#[derive(Debug)]
enum RunStatus {
    OK = 0,
    Error = 1,
}

pub trait ApplicationRunner {
    type Error: std::error::Error;
    type CmdArgs: IntoApp + Clap + Debug;

    /// * Configures logger (default - when using `simple_logging` feature or user defined -
    /// when they override [`ApplicationRunner::configure_logging`] method in their trait implementation
    /// * Parses Clap command line arguments
    /// * Runs application, then returns OK or error status and prints possible error
    fn main(&self) -> i32 {
        self.configure_logging();

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

    /// Main method of the application, everything that user wants to be run must be put into this method
    ///
    /// # Arguments
    ///
    /// * cmd_args - Clap command line arguments
    fn run(&self, cmd_args: Self::CmdArgs) -> Result<(), Self::Error>;

    /// Checks if error message should be printed using red color
    fn should_write_app_error_message_with_colors(&self) -> bool {
        Self::CmdArgs::into_app().is_set(AppSettings::ColoredHelp)
    }

    /// Writes error message
    ///
    /// # Arguments
    ///
    /// * error_message - error text
    fn write_app_error_message(&self, error_message: &String) {
        use crate::write_colored_error_message;

        if self.should_write_app_error_message_with_colors() {
            write_colored_error_message(error_message).unwrap();
        } else {
            eprintln!("{}", error_message);
        }
    }

    /// Initializes logger
    /// With feature `simple_logging` it starts `flexi_logger`
    ///
    /// On default, it has empty implementation, so nothing will be logged. User can use their own logger by overriding
    /// this method
    fn configure_logging(&self) {
        #[cfg(feature = "simple_logging")] {
            flexi_logger::Logger::try_with_env().unwrap().log_to_stderr().start().unwrap();
            info!("Default logger initialized");
        }
    }
}