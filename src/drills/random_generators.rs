use rand::{
    distr::{Distribution, Uniform},
    Rng,
};

pub(super) fn runway<R: Rng>(rng: &mut R) -> u32 {
    let dist = Uniform::new_inclusive(1, 36).unwrap();
    dist.sample(rng)
}

pub(super) fn direction<R: Rng>(rng: &mut R) -> f64 {
    let dist = Uniform::new(0, 360).unwrap();
    dist.sample(rng) as f64
}

pub(super) fn speed<R: Rng>(rng: &mut R) -> f64 {
    let dist = Uniform::new_inclusive(75, 150).unwrap();
    dist.sample(rng) as f64
}

pub(super) fn wind_speed<R: Rng>(rng: &mut R) -> f64 {
    let dist = Uniform::new_inclusive(0, 35).unwrap();
    dist.sample(rng) as f64
}

pub(super) fn track_distance<R: Rng>(rng: &mut R) -> f64 {
    let dist = Uniform::new_inclusive(50, 200).unwrap();
    dist.sample(rng) as f64
}

pub(super) fn track_distance_already_covered<R: Rng>(rng: &mut R, length: f64) -> f64 {
    let dist = Uniform::new_inclusive(0.2, 0.8).unwrap();
    let portion = dist.sample(rng);
    (length * portion).round()
}

pub(super) fn distance_off_track<R: Rng>(rng: &mut R) -> f64 {
    let dist = Uniform::new_inclusive(1, 15).unwrap();
    dist.sample(rng) as f64
}
