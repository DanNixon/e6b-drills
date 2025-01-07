mod calculator;
mod drills;

use crate::CliRun;
use clap::Subcommand;

#[derive(Subcommand)]
pub(super) enum Command {
    Calculator(calculator::Command),
    Drills(drills::Command),
}

impl CliRun for Command {
    fn run(&self) {
        match &self {
            Command::Calculator(command) => command.run(),
            Command::Drills(command) => command.run(),
        }
    }
}
