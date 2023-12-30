use clap::{Args, Subcommand};

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
#[command(flatten_help = true)]
pub struct ProblemArgs {
    #[command(subcommand)]
    pub command: Option<ProblemCommands>,
}

#[derive(Debug, Subcommand)]
pub enum ProblemCommands {
    Init { name: String },
    Submit { name: String },
}
