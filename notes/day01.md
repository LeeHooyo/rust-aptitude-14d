# Day 01 - 환경세팅 & 첫 컴파일
**Objectives:**
- Rust 개발환경 세팅 및 workspace 구조 이해
- Cargo workspace의 개념 익히기

**Concepts:** `rustup`, `cargo fmt`, `clippy`, `workspace`, `crate`

---

## Commands
```bash
brew install helix
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new labs/day01_hello --vsc none
cargo run -p day01_hello
cargo fmt --all
cargo clippy --all -targers -- -D warnings

