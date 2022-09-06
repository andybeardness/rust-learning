use std::f64::consts;

fn main() {
    let x = 2.0 * consts::PI;
    let diff = x.cos() - 1.0;
    let abs_diff = diff.abs();

    assert!(abs_diff < 1e-10);
}
