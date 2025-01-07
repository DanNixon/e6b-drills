pub(crate) fn wca(hdg: f64, trk: f64) -> f64 {
    hdg - trk
}

pub(crate) fn w(wca: f64, gs: f64, trk: f64, tas: f64) -> f64 {
    let wca = wca.to_radians();
    let trk = trk.to_radians();

    (trk + (tas * wca.sin()).atan2((tas * wca.cos()) - gs)).to_degrees() % 360.
}

pub(crate) fn wv(wca: f64, gs: f64, tas: f64) -> f64 {
    let wca = wca.to_radians();

    ((tas - gs).powf(2.) + (4. * tas * gs * (wca / 2.).sin().powf(2.))).sqrt()
}
