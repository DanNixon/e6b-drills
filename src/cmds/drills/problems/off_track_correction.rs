use super::Problem;
use crate::{calculators::one_in_sixty_rule, cmds::drills::random_generators};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt::{Display, Write};

pub(super) struct OffTrackCorrection {
    track_length: f64,

    track_covered: f64,

    distance_off_track: f64,
    direction_off_track: Direction,

    track_error: f64,
    closing_angle: f64,
}

impl Problem for OffTrackCorrection {
    fn new_random<R: Rng>(rng: &mut R) -> Self {
        let track_length = random_generators::track_distance(rng);
        let distance_off_track = random_generators::distance_off_track(rng);
        let direction_off_track = rng.gen();
        let track_covered = random_generators::track_distance_already_covered(rng, track_length);

        let track_error = one_in_sixty_rule::calculate_angle(distance_off_track, track_covered);

        let track_remaining = track_length - track_covered;
        let closing_angle = one_in_sixty_rule::calculate_angle(distance_off_track, track_remaining);

        Self {
            track_length,
            track_covered,
            distance_off_track,
            direction_off_track,
            track_error,
            closing_angle,
        }
    }

    fn problem(&self) -> String {
        let mut s = String::new();
        writeln!(s, "Following a track between points {}nm apart, after {}nm the aircraft is {}nm to the {} of the track.", self.track_length, self.track_covered, self.distance_off_track, self.direction_off_track).unwrap();
        write!(
            s,
            "What adjustment to heading must be made to rejoin the track by the destination point?"
        )
        .unwrap();
        s
    }

    fn solution(&self) -> String {
        let mut s = String::new();
        writeln!(s, "TE = {:.1}", self.track_error).unwrap();
        writeln!(s, "CA = {:.1}", self.closing_angle).unwrap();
        write!(
            s,
            "Adjust heading by {:.1} to the {}",
            self.track_error + self.closing_angle,
            self.direction_off_track.opposite(),
        )
        .unwrap();
        s
    }
}

enum Direction {
    Left,
    Right,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Left => "left",
                Direction::Right => "right",
            }
        )
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        let index: u8 = rng.gen_range(0..2);
        match index {
            0 => Direction::Left,
            1 => Direction::Right,
            _ => unreachable!(),
        }
    }
}
