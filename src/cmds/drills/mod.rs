mod problems;
mod random_generators;

use crate::CliRun;
use clap::Parser;
use crossterm::style::Stylize;
use inquire::Confirm;
use problems::ProblemTypes;
use rand::Rng;

/// Show a series of calculation problems.
/// Data is randomly generated, it should be able to be correctly calculated, but may not reflect a
/// realistic flight scenario.
/// Values are chosen to be mostly correct for typical training aircraft.
#[derive(Parser)]
pub(crate) struct Command {}

impl CliRun for Command {
    fn run(&self) {
        let mut rng = rand::thread_rng();

        loop {
            let p: ProblemTypes = rng.gen();
            let p = p.new_random(&mut rng);

            println!("{}", p.problem().blue());

            if !Confirm::new("Continue?").prompt().unwrap() {
                break;
            }

            println!("{}\n", p.solution().magenta());
        }
    }
}
