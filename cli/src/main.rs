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
                Some(ProblemCommands::Init { number }) => {
                    problem_init(number);
                }
                Some(ProblemCommands::Submit { number }) => {
                    problem_submit(number);
                }
                None => todo!(),
            }
        }
    }
}
