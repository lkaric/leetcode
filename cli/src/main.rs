pub mod problem;

use problem::clap::{ProblemArgs, ProblemCommands};
use problem::handler::{problem_init, problem_submit};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "leetcode")]
#[command(about = "Leetcode CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command()]
    Problem(ProblemArgs),
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Problem(problem) => {
            let problem_cmd = problem.command;

            match problem_cmd {
                Some(ProblemCommands::Init { name }) => {
                    problem_init(name);
                }
                Some(ProblemCommands::Submit { name }) => {
                    problem_submit(name);
                }
                None => todo!(),
            }
        }
    }
}
