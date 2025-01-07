pub(crate) fn calculate_angle(distance_off_track: f64, track_run: f64) -> f64 {
    (distance_off_track * 60.) / track_run
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fundamental_1() {
        let distance_off_track = 1.;
        let track_run = 60.;
        let track_error = calculate_angle(distance_off_track, track_run);
        assert_eq!(track_error, 1.);
    }

    #[test]
    fn fundamental_5() {
        let distance_off_track = 5.;
        let track_run = 60.;
        let track_error = calculate_angle(distance_off_track, track_run);
        assert_eq!(track_error, 5.);
    }

    #[test]
    fn calc_1() {
        let distance_off_track = 2.;
        let track_run = 25.;
        let track_error = calculate_angle(distance_off_track, track_run);
        assert_eq!(track_error, 4.8);
    }
}
