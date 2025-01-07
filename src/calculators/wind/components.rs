#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum ParallelWind {
    Head(f64),
    Tail(f64),
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum CrossWind {
    Left(f64),
    Right(f64),
}

pub(crate) fn runway_heading(runway: u32) -> f64 {
    runway as f64 * 10.
}

pub(crate) fn parallel(heading: f64, w: f64, wv: f64) -> ParallelWind {
    let heading = heading.to_radians();
    let w = w.to_radians();

    let dotprod = heading.sin() * w.sin() + heading.cos() * w.cos();
    let theta = dotprod.acos();

    let pw = wv * theta.cos();

    if pw < 0.0 {
        ParallelWind::Tail(pw.abs())
    } else {
        ParallelWind::Head(pw.abs())
    }
}

pub(crate) fn cross(heading: f64, w: f64, wv: f64) -> CrossWind {
    let relative = heading - w;

    let heading = heading.to_radians();
    let w = w.to_radians();

    let dotprod = heading.sin() * w.sin() + heading.cos() * w.cos();
    let theta = dotprod.acos();

    let xw = wv * theta.sin();

    if relative < 0. {
        CrossWind::Right(xw)
    } else {
        CrossWind::Left(xw)
    }
}
