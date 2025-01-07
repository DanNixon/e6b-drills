mod wind_components;
mod wind_hdg_gs;
mod wind_w_v;

use crate::CliRun;
use clap::{Parser, Subcommand};

/// Perform single calculations
#[derive(Parser)]
pub(crate) struct Command {
    #[command(subcommand)]
    calculator: Calculator,
}

impl CliRun for Command {
    fn run(&self) {
        self.calculator.run();
    }
}

#[derive(Subcommand)]
pub(crate) enum Calculator {
    WindHdgGs(wind_hdg_gs::Command),
    WindWV(wind_w_v::Command),
    WindComponents(wind_components::Command),
}

impl CliRun for Calculator {
    fn run(&self) {
        match self {
            Calculator::WindHdgGs(command) => command.run(),
            Calculator::WindWV(command) => command.run(),
            Calculator::WindComponents(command) => command.run(),
        }
    }
}
