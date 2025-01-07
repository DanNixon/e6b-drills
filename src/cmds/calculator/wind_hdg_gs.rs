use crate::CliRun;
use clap::Parser;

#[derive(Parser)]
pub(crate) struct Command {
    #[arg(long)]
    trk: f64,

    #[arg(long)]
    tas: f64,

    #[arg(long, short)]
    wind_direction: f64,

    #[arg(long, short = 'v')]
    wind_speed: f64,
}

impl CliRun for Command {
    fn run(&self) {
        let wca = crate::calculators::wind::hdg_gs::wca(
            self.wind_direction,
            self.wind_speed,
            self.trk,
            self.tas,
        );

        let hdg = crate::calculators::wind::hdg_gs::hdg(self.trk, wca);

        let gs = crate::calculators::wind::hdg_gs::gs(
            self.wind_direction,
            self.wind_speed,
            self.trk,
            self.tas,
            wca,
        );

        let wca = wca.round();
        let hdg = hdg.round();
        let gs = gs.round();

        println!("WCA: {wca}");
        println!("HDG: {hdg}");
        println!("GS: {gs}");
    }
}
