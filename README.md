# Rust Aptitude 14-Day Project

**Rust 기초 감각과 언어 철학을 체득하기 위한 14일 학습 로드맵**

> "Write code that feels safe, performs fast, and reads clean."

---

## Structure
rust-aptitude-14d/
├── Cargo.toml            # Workspace definition
├── notes/                # Obsidian 학습 노트 (day01~day14)
├── labs/                 # 일자별 실습 크레이트
│   ├── day01_hello/
│   ├── day02_ownership/
│   ├── day03_shadow/
│   ├── day04_functions/
│   ├── day05_structs/
│   └── day06_enum_match/
└── projects/
    └── cli_text_analyzer/ # Day14 통합 실습 프로젝트

---

## Daily Topics

| Day | Topic | Core Concepts |
|-----|-------|----------------|
| 01 | Hello, Rust | Cargo, println!, String |
| 02 | Ownership & Borrowing | move, borrow, lifetime |
| 03 | Mutability & Shadowing | let mut, scope, UTF-8 |
| 04 | Functions & Lifetimes | 참조 반환, 라이프타임 규칙 |
| 05 | Structs & Traits | impl, trait, method |
| 06 | Enums & Pattern Matching | enum, match, Option, Result |
| 07 | Collections & Iterators | Vec, HashMap, iter/map/filter |
| 08 | Error Handling | Result, ?, unwrap_or_else |
| 09 | Modules & Crates | mod, pub, use, 구조화 |
| 10 | File & IO Handling | fs, BufReader, CLI IO |
| 11 | Concurrency & Threads | thread::spawn, join |
| 12 | Smart Pointers | Box, Rc, RefCell |
| 13 | Async & Await | Future, tokio |
| 14 | CLI Mini Project | cli_text_analyzer 완성 |

---

## Notes
각 Day는 다음 형식의 Obsidian 노트로 구성됨:
notes/dayXX.md
포함 항목:
- **Objectives** (학습 목표)
- **Commands** (터미널 명령어)
- **Notes** (핵심 개념 요약)
- **Errors & Fixes**
- **Reflection**

---

## Run Examples
cargo run -p dayXX
cargo test -p dayXX

---

## Philosophy
- **Safety**: 소유권과 라이프타임을 통한 메모리 안정성  
- **Clarity**: 암묵적 동작보다 명시적 표현  
- **Performance**: 제로 비용 추상화(Zero-cost abstraction)

---

## License
MIT License  
© 2025, Yunkyu Lee

