use clap::Parser;

use crate::CliRun;

#[derive(Parser)]
pub(crate) struct Command {
    #[arg(long)]
    runway: Option<u32>,

    #[arg(long)]
    heading: Option<f64>,

    #[arg(long, short)]
    wind_direction: f64,

    #[arg(long, short = 'v')]
    wind_speed: f64,
}

impl CliRun for Command {
    fn run(&self) {
        if self.runway.is_some() == self.heading.is_some() {
            panic!("Need either runway or heading");
        }

        let hdg = self
            .heading
            .unwrap_or(crate::calculators::wind::components::runway_heading(
                self.runway.unwrap(),
            ));

        let p = crate::calculators::wind::components::parallel(
            hdg,
            self.wind_direction,
            self.wind_speed,
        );

        let x =
            crate::calculators::wind::components::cross(hdg, self.wind_direction, self.wind_speed);

        println!("Parallel: {p:?}");
        println!("Cross: {x:?}");
    }
}
