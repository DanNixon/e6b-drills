mod problems;
mod random_generators;

use crossterm::style::Stylize;
use inquire::Confirm;
use problems::ProblemTypes;
use rand::Rng;

pub(super) fn run() {
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
