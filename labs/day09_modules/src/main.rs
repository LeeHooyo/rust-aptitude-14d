use mathlib::{geometry, utils, version};

fn main() {
    println!("=== [Day09 Modules & Crates Demo] ===");
    println!("Using {}", version());

    geometry::describe();
    let area = geometry::area::rectangle(10.0, 5.0);
    let vol = geometry::volume::sphere(3.0);

    println!("Area: {}", utils::round2(area));
    println!("Volume: {}", utils::round2(vol));
}

