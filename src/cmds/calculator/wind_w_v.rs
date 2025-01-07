use crate::CliRun;
use clap::Parser;

#[derive(Parser)]
pub(crate) struct Command {
    #[arg(long)]
    hdg: f64,

    #[arg(long)]
    tas: f64,

    #[arg(long)]
    trk: f64,

    #[arg(long)]
    gs: f64,
}

impl CliRun for Command {
    fn run(&self) {
        let wca = crate::calculators::wind::w_v::wca(self.hdg, self.trk);
        let w = crate::calculators::wind::w_v::w(wca, self.gs, self.trk, self.tas);
        let v = crate::calculators::wind::w_v::wv(wca, self.gs, self.tas);

        let w = w.round();
        let v = v.round();

        println!("Wind: {w}/{v}");
    }
}
