use clap::{crate_version, AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = crate_version!(), setting=AppSettings::ColoredHelp)]
pub struct CmdArgs {}
