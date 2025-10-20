mod utils;
mod math;
mod io_layer;

use math::area;
use utils::greet;
use io_layer::logger;

fn main() {
    greet("Ferris");
    let (w, h) = (10.0, 20.0);
    println!("사각형 넓이: {}", area::rectangle(w, h));
    println!("삼각형 넓이: {}", area::triangle(w, h));

    // private 모듈 접근 시도 (주석 참고)
    // io_layer::secret::hidden(); // private module access

    logger::info("계산 완료");
}
