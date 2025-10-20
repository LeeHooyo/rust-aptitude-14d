pub fn cube(a: f64) -> f64 {
    a.powi(3)
}
pub fn sphere(r: f64) -> f64 {
    const PI: f64 = 3.1415926535;
    4.0 / 3.0 * PI * r.powi(3)
}

