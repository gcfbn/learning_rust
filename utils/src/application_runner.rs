use clap::{AppSettings, Clap, IntoApp};

enum RunStatus {
    OK = 0,
    Error = 1,
}

pub trait ApplicationRunner {
    type Error: std::error::Error;
    type CmdArgs: IntoApp + Clap;

    fn main(&self) -> i32 {
        let cmd_args = Self::CmdArgs::parse();
        let status = if let Err(err) = self.run(cmd_args) {
            self.write_app_error_message(&err.to_string());
            RunStatus::Error
        } else {
            RunStatus::OK
        };

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
