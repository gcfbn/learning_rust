use clap::{AppSettings, Clap};
use utils::PathBufWithFileThatMustExist;

/// Subcommand running Kruskal's algorithm for graph built from `task_file`
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct RunAlgorithmArgs {
    /// Name of file containing graph data
    #[clap(long, short)]
    pub task_file: PathBufWithFileThatMustExist,
}
