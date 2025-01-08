mod calculators;
mod drills;

use clap::Parser;

/// A very basic tool to help learning calculations required for the PPL navigation exam, mostly
/// around use of a flight computer such as an E6B or CRP-1.
/// Please double check anything that seems wrong using a more established tool such as E6BX
/// (https://e6bx.com/).
/// Data is randomly generated, it should be able to be correctly calculated, but may not reflect a
/// realistic flight scenario.
/// Values are chosen to be mostly correct for typical training aircraft.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {}

fn main() {
    let _ = Cli::parse();
    crate::drills::run();
}
