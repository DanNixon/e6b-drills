use super::{super::random_generators, Problem};
use crate::calculators::wind::components::{CrossWind, ParallelWind};
use rand::Rng;
use std::fmt::Write;

pub(super) struct WindComponents {
    runway: u32,
    w: f64,
    wv: f64,

    p: ParallelWind,
    x: CrossWind,
}

impl Problem for WindComponents {
    fn new_random<R: Rng>(rng: &mut R) -> Self {
        let runway = random_generators::runway(rng);

        let w = random_generators::direction(rng);
        let wv = random_generators::wind_speed(rng);

        let heading = crate::calculators::wind::components::runway_heading(runway);

        let p = crate::calculators::wind::components::parallel(heading, w, wv);
        let x = crate::calculators::wind::components::cross(heading, w, wv);

        Self {
            runway,
            w,
            wv,
            p,
            x,
        }
    }

    fn problem(&self) -> String {
        format!(
            "Given wind {}/{}kt, calculate the wind components for runway {}.",
            self.w, self.wv, self.runway
        )
    }

    fn solution(&self) -> String {
        let mut s = String::new();

        match self.p {
            ParallelWind::Head(v) => writeln!(s, "{:.0}kt headwind", v).unwrap(),
            ParallelWind::Tail(v) => writeln!(s, "{:.0}kt tailwind", v).unwrap(),
        }

        match self.x {
            CrossWind::Left(v) => write!(s, "{:.0}kt crosswind from left", v).unwrap(),
            CrossWind::Right(v) => write!(s, "{:.0}kt crosswind from right", v).unwrap(),
        }

        s
    }
}
