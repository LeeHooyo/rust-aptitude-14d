// geometry/area.rs 가 아니라 math/area.rs 임 (심화에서 하위 모듈로 확장 예정)
pub fn rectangle(w: f64, h: f64) -> f64 {
    w * h
}

pub fn triangle(w: f64, h: f64) -> f64 {
    w * h / 2.0
}
