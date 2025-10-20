# Day09 — Modules & Crates

**Objectives:**  
- Rust의 모듈 시스템(`mod`, `pub`, `use`) 이해  
- crate(패키지) 단위의 의존성 구조 설계  
- 라이브러리 crate(`lib.rs`)를 workspace에서 참조하는 방법 체득  
- `pub(crate)`, `pub(super)`를 통한 접근 제어 익히기  
- 실제 Substrate-style 구조 감각 익히기  

**Concepts:** `mod`, `pub`, `crate`, `use`, `super`, `self`, `pub(crate)`, `path dependency`, `workspace`

---

## Commands
```bash
cargo new labs/day09_modules --bin
cargo new labs/day09_modules/mathlib --lib
cargo run -p day09_modules
cargo test -p day09_modules
```

## Notes

### 1. 모듈과 크레이트의 차이

- **모듈(mod)** → 파일 단위, 코드의 *논리적 그룹화*  
- **크레이트(crate)** → 컴파일 단위, 빌드 가능한 독립 패키지  

즉, 하나의 crate 안에 여러 모듈이 있고,  
workspace는 여러 crate을 관리하는 상위 구조다.

### 2. 모듈 기본 구조

```rust
// src/main.rs
mod utils;
mod math;
mod io_layer;

use math::geometry::area;
use utils::greet;
use io_layer::logger;

fn main() {
    greet("Ferris");
    let (w, h) = (10.0, 20.0);
    println!("사각형 넓이: {}", area::rectangle(w, h));
    println!("삼각형 넓이: {}", area::triangle(w, h));
    logger::info("계산 완료!");
}
```

### 3. 폴더 기반 모듈 구성

```plaintext
labs/day09_modules/
├── src/
│   ├── main.rs
│   ├── utils.rs
│   ├── math/
│   │   ├── mod.rs
│   │   ├── area.rs
│   │   └── volume.rs
│   └── io_layer/
│       ├── mod.rs
│       ├── logger.rs
│       └── secret.rs
```

각각의 `mod.rs`는 “해당 디렉토리의 루트 모듈 역할”을 하며,  
하위 파일들을 `pub mod ...`로 불러온다.

### 4. Crate 간 분리 (심화)

별도의 라이브러리 crate(`mathlib`)를 생성하고 workspace에서 참조한다.

#### 루트 Cargo.toml
```toml
[workspace]
members = [
  "labs/day09_modules",
  "labs/day09_modules/mathlib"
]
resolver = "2"
```

#### day09_modules/Cargo.toml
```toml
[package]
name = "day09_modules"
version = "0.1.0"
edition = "2021"

[dependencies]
mathlib = { path = "mathlib" }
```

이렇게 해야 `mathlib` crate가 인식되고,  
`use mathlib::geometry;` 형태로 불러올 수 있다.

### 5. mathlib 구조

```plaintext
mathlib/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── geometry/
    │   ├── mod.rs
    │   ├── area.rs
    │   └── volume.rs
    └── utils.rs
```

#### lib.rs
```rust
pub mod geometry;
pub mod utils;

pub fn version() -> &'static str {
    "mathlib v1.0.0"
}
```

#### geometry/mod.rs
```rust
pub mod area;
pub mod volume;

pub fn describe() {
    println!("[Geometry] Area & Volume calculations");
}
```

#### geometry/area.rs
```rust
pub fn rectangle(w: f64, h: f64) -> f64 {
    w * h
}
pub fn triangle(w: f64, h: f64) -> f64 {
    w * h / 2.0
}
```

#### geometry/volume.rs
```rust
pub fn cube(a: f64) -> f64 {
    a.powi(3)
}
pub fn sphere(r: f64) -> f64 {
    const PI: f64 = 3.1415926535;
    4.0 / 3.0 * PI * r.powi(3)
}
```

#### utils.rs
```rust
pub fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}
```

### 6. main.rs (최종)
```rust
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
```

### 7. 실행 결과

```bash
cargo run -p day09_modules
```

출력:
```
=== [Day09 Modules & Crates Demo] ===
Using mathlib v1.0.0
[Geometry] Area & Volume calculations
Area: 50
Volume: 113.1
```

---

## Errors & Fixes

- ❌ `use of unresolved module or unlinked crate 'mathlib'`  
  **원인:** dependency 선언 누락 (`Cargo.toml`에 path 미등록)  
  **해결:**  
  ~~~toml
  [dependencies]
  mathlib = { path = "mathlib" }
  ~~~

- ⚠️ `file not found for module 'math'`  
  **원인:** `mod math;` 선언했지만 실제 디렉토리 이름/파일 불일치  
  **해결:** `src/math/mod.rs` 경로 확인 및 일치시킬 것

---

## Reflection

- `mod`는 단순히 *코드 파일 구조화*,  
  `crate`는 *빌드/배포 단위*.  
  둘은 계층적으로 다르지만 구조적으로 긴밀하게 연결된다.  

- `pub`, `pub(crate)`, `pub(super)`는 **캡슐화 수준**을 조절하는 도구다.  
  Substrate나 Solana 같은 대형 프레임워크도 이 계층화를 기반으로 동작한다.  

- Rust의 모듈 시스템은 “자동 인식”이 아니라 **명시적 설계**를 강제한다.  
  → 이게 러스트 프로젝트의 유지보수성과 보안성을 동시에 높여주는 핵심 철학이다.

