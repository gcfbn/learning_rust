use clap::Clap;
use runner::*;
use std::io::{self, Write};
use std::process;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

type Result<T, E = RunnerError> = std::result::Result<T, E>;

fn write_error_message(msg: &str) -> io::Result<()> {
    let choice = if atty::is(atty::Stream::Stderr) {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut out = StandardStream::stderr(choice);
    out.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
    write!(&mut out, "error: ")?;
    out.set_color(ColorSpec::new().set_fg(Some(Color::White)))?;
    writeln!(&mut out, "{}", msg)
}

/// Main function that is called when the app starts
///
/// Calls run() function and kills process if it returns an error
fn main() {
    if let Err(err) = run() {
        write_error_message(&err.to_string()).unwrap();
        process::exit(1);
    }
}

/// Builds graph from given file and calculates weight of it's minimum spanning tree, returns [`anyhow::Result`]
fn run() -> Result<()> {
    let cmd_args: CmdArgs = CmdArgs::parse();

    match cmd_args.subcommand {
        SubCommand::RunAlgorithm(task_data) => {
            let graph = graph::build_graph(&task_data.task_file)?;
            let output = algorithms::calculate_min_total_weight(graph);
            println!("{}", output);
        }

        SubCommand::GenerateGraphFile(params) => {
            generate_graph(&params)?;
            println!("Graph file with path {:?} successfully generated!", params.graph_file);
        }
    }
    Ok(())
}
