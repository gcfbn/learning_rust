use clap::{AppSettings, Clap};
use utils::{PathBufWithFileThatMustExist, PositiveInteger};

/// Subcommand running one of the available algorithms for graph built from `task_file`
#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct RunAlgorithmArgs {
    /// Name of file containing graph data
    #[clap(long, short)]
    pub task_file: PathBufWithFileThatMustExist,

    /// Algorithm name
    #[clap(subcommand)]
    pub algorithm_args: AlgorithmArgs,
}

/// Available algorithms and their args (if necessary)
#[derive(Clap, Debug)]
pub enum AlgorithmArgs {
    /// Calculates weight of graph minimum spanning tree
    #[clap(visible_alias = "k")]
    Kruskals {},

    /// Calculates shortest path weight from `start_node` to `end_node`
    #[clap(visible_alias = "d")]
    Dijkstras {
        #[clap(long, short)]
        start_node: PositiveInteger,
        #[clap(long, short)]
        end_node:   PositiveInteger,
    },
}
