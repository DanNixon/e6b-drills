mod calculators;
mod cmds;

use clap::Parser;

trait CliRun {
    fn run(&self);
}

/// A very basic tool to help learning calculations required for the PPL navigation exam, mostly
/// around use of a flight computer such as an E6B or CRP-1.
/// Please double check anything that seems wrong using a more established tool such as E6BX
/// (https://e6bx.com/).
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: cmds::Command,
}

impl CliRun for Cli {
    fn run(&self) {
        self.command.run();
    }
}

fn main() {
    let cli = Cli::parse();
    cli.run();
}
