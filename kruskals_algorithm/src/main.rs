use anyhow::{anyhow, Result as aResult};
use clap::{AppSettings, Clap};
use kruskals_algorithm::LibResult;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    if let Err(err) = run() {
        println!("Error: {:?}", err);
        process::exit(1);
    }
}

#[derive(Debug, Clap)]
#[clap(
    name = "kruskal_algorithm",
    version = "1.0",
    about = "Algorithms & Data structures task from graph theory",
    author = "Bartek M. <bmekarski@interia.pl>",
    setting=AppSettings::ColoredHelp,
)]
struct CmdArgs {
    /// Task file with task data
    #[clap(long, short, parse(from_os_str), validator(file_exists))]
    pub task_file: PathBuf,
}

fn file_exists(p: &str) -> aResult<()> {
    if Path::new(p).exists() {
        Ok(())
    } else {
        Err(anyhow!("the file does not exist: {}", p))
    }
}

fn run() -> LibResult<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    let output = kruskals_algorithm::run(cmd_args.task_file)?;
    println!("{}", output);

    Ok(())
}
