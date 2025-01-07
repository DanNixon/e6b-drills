pub(crate) fn wca(w: f64, wv: f64, trk: f64, tas: f64) -> f64 {
    let w = w.to_radians();
    let trk = trk.to_radians();

    ((wv * (w - trk).sin()) / tas).asin().to_degrees()
}

pub(crate) fn hdg(trk: f64, wca: f64) -> f64 {
    trk + wca
}

pub(crate) fn gs(w: f64, wv: f64, trk: f64, tas: f64, wca: f64) -> f64 {
    let w = w.to_radians();
    let trk = trk.to_radians();
    let wca = wca.to_radians();

    (tas.powf(2.) + wv.powf(2.) - (2. * tas * wv * (trk - w + wca).cos())).sqrt()
}
