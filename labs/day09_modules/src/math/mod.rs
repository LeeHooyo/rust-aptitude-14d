pub mod area;
pub mod volume;

// 내부 공통 상수
const PI: f64 = 3.1415926535;

pub fn circle_area(r: f64) -> f64 {
    PI * r * r
}
