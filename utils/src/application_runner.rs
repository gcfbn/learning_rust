use clap::{AppSettings, Clap, IntoApp};
use log::{error, info, trace};
use std::fmt::{Debug, Display};

#[derive(Debug)]
enum RunStatus {
    OK    = 0,
    Error = 1,
}

pub trait AppLoggerCreator {
    fn new() -> Self;
}

#[cfg(feature = "default_logging")]
pub struct DefaultAppLoggerCreator {
    _handle: flexi_logger::LoggerHandle,
}

#[cfg(feature = "default_logging")]
impl AppLoggerCreator for DefaultAppLoggerCreator {
    fn new() -> Self {
        use flexi_logger::{detailed_format, Duplicate, FileSpec, Logger};

        let _handle = Logger::try_with_env_or_str("warn")
                    .unwrap()
                    .log_to_file(FileSpec::default().directory(".logs"))
                    .duplicate_to_stderr(Duplicate::Warn)
                    // .duplicate_to_stderr(Duplicate::All)
                    .format_for_files(detailed_format)
                    .format_for_stderr(default_colored_format)
                    .print_message()
                    .create_symlink("current_run") // create a symbolic link to the current log file
                    .start()
                    .unwrap();

        info!("default logger initialized");

        DefaultAppLoggerCreator { _handle }
    }
}

pub trait ApplicationRunner {
    type Error: Display;
    type CmdArgs: IntoApp + Clap + Debug;
    type AppLoggerCreator: AppLoggerCreator;

    /// * Configures logger (default - when using `default_logging` feature or user defined -
    ///   when they override [`ApplicationRunner::configure_logging`] method in their trait implementation
    /// * Parses Clap command line arguments
    /// * Runs application, then returns OK or error status and prints possible error
    fn main(&self) -> i32 {
        let status;

        {
            let _app_logger_creator = Self::AppLoggerCreator::new();

            let cmd_args = Self::CmdArgs::parse();
            trace!("parsed command line arguments - {:?}", cmd_args);

            // maybe with application name or with chosen subcommand
            info!("running application...");

            status = if let Err(err) = self.run(cmd_args) {
                let error_message = &err.to_string();

                self.write_app_error_message(error_message);

                error!("{}", error_message);

                RunStatus::Error
            } else {
                RunStatus::OK
            };

            info!("closing application with status {:?}", &status);
        }

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
    fn write_app_error_message(&self, error_message: &str) {
        use crate::write_colored_error_message;

        if self.should_write_app_error_message_with_colors() {
            write_colored_error_message(error_message).unwrap();
        } else {
            eprintln!("{}", error_message);
        }
    }
}

// -----------------------------------------------------------------------------

#[cfg(feature = "default_logging")]
use flexi_logger::{style, DeferredNow, Record};

#[cfg(feature = "default_logging")]
pub fn default_colored_format(
    w: &mut dyn std::io::Write,
    now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();

    write!(
        w,
        "{} {:>5}: {}",
        now.now().format("%Y-%m-%d %H:%M:%S%.6f"),
        style(level, level),
        &record.args()
    )
}

// -----------------------------------------------------------------------------
