mod heading_and_groundspeed;
mod off_track_correction;
mod wind;
mod wind_components;

use self::{
    heading_and_groundspeed::HeadingAndGroundspeed, off_track_correction::OffTrackCorrection,
    wind::Wind, wind_components::WindComponents,
};
use rand::{distributions::Distribution, distributions::Standard, Rng};

#[derive(Debug, Clone)]
pub(crate) enum ProblemTypes {
    HeadingAndGroundspeed,
    Wind,
    WindComponents,
    OffTrackCorrection,
}

impl ProblemTypes {
    pub(crate) fn new_random<R: Rng>(&self, rng: &mut R) -> Box<dyn Problem> {
        match self {
            ProblemTypes::HeadingAndGroundspeed => Box::new(HeadingAndGroundspeed::new_random(rng)),
            ProblemTypes::Wind => Box::new(Wind::new_random(rng)),
            ProblemTypes::WindComponents => Box::new(WindComponents::new_random(rng)),
            ProblemTypes::OffTrackCorrection => Box::new(OffTrackCorrection::new_random(rng)),
        }
    }
}

impl Distribution<ProblemTypes> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ProblemTypes {
        let index: u8 = rng.gen_range(0..4);
        match index {
            0 => ProblemTypes::HeadingAndGroundspeed,
            1 => ProblemTypes::Wind,
            2 => ProblemTypes::WindComponents,
            3 => ProblemTypes::OffTrackCorrection,
            _ => unreachable!(),
        }
    }
}

pub(crate) trait Problem {
    fn new_random<R: Rng>(rng: &mut R) -> Self
    where
        Self: Sized;

    fn problem(&self) -> String;
    fn solution(&self) -> String;
}
