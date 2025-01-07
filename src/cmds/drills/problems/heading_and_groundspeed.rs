use super::{super::random_generators, Problem};
use crate::calculators::wind::hdg_gs;
use rand::Rng;
use std::fmt::Write;

pub(super) struct HeadingAndGroundspeed {
    hdg: f64,
    tas: f64,

    trk: f64,
    gs: f64,

    w: f64,
    wv: f64,
}

impl Problem for HeadingAndGroundspeed {
    fn new_random<R: Rng>(rng: &mut R) -> Self {
        let tas = random_generators::speed(rng);
        let trk = random_generators::direction(rng);

        let w = random_generators::direction(rng);
        let wv = random_generators::wind_speed(rng);

        let wca = hdg_gs::wca(w, wv, trk, tas);
        let hdg = hdg_gs::hdg(trk, wca);
        let gs = hdg_gs::gs(w, wv, trk, tas, wca);

        Self {
            hdg,
            tas,
            trk,
            gs,
            w,
            wv,
        }
    }

    fn problem(&self) -> String {
        let mut s = String::new();
        writeln!(s, "Given:").unwrap();
        writeln!(s, " Track: {}", self.trk).unwrap();
        writeln!(s, " TAS: {}kt", self.tas).unwrap();
        writeln!(s, " Wind: {}/{}kt", self.w, self.wv).unwrap();
        write!(s, "Calculate HDG and GS.").unwrap();
        s
    }

    fn solution(&self) -> String {
        let mut s = String::new();
        writeln!(s, "HDG: {:.0}", self.hdg).unwrap();
        write!(s, "GS: {:.0}kt", self.gs).unwrap();
        s
    }
}
