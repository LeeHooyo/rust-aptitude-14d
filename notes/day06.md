# Day06 — Enums & Pattern Matching

**Objectives:**  
- 여러 상태를 `enum`으로 안전하게 표현한다.  
- `match` 문으로 모든 경우를 명시적으로 처리한다.  
- `Option` / `Result` 타입을 이해하고 `if let`으로 간결하게 다룬다.

**Concepts:** `enum`, `match`, `Option<T>`, `Result<T, E>`, `_`, `if let`

---

## Commands
```bash
cargo run -p day06_enum_match
cargo test -p day06_enum_match
``

## Notes
- `enum`은 C의 단순한 상수 집합이 아니라, **데이터를 담을 수 있는 타입**.  
- `match`는 반드시 **모든 경우를 처리**해야 하므로 런타임 누락이 없다.  
- `Option`은 null의 안전한 대체재. (`Some`, `None`)  
- `Result`는 예외 처리 대신 사용하는 패턴 (`Ok`, `Err`).  
- `if let`, `while let`은 특정 패턴만 다루고 싶을 때 유용.  
- `match`는 패턴 매칭이므로 값 비교를 넘어서 **구조 해체 + 조건 분기** 기능을 동시에 수행한다.

## Errors & Fixes
- ❌ **non-exhaustive patterns: `ConnectionState` not covered**  
  → 모든 경우를 `match` 문에서 다루지 않았다는 뜻. `_ =>` 또는 빠진 variant 추가 필요.  
- ❌ **mismatched types: expected `Result`, found `Option`**  
  → 타입 불일치. 함수 반환 타입과 매칭 구문 확인 필요.  
- ❌ **unused variable warnings**  
  → `_` prefix나 `_` 패턴으로 무시 가능.

## Reflection
- Rust의 `enum`은 단순한 열거가 아니라 **상태 머신(state machine)** 구현에 가깝다.  
- `match`의 강제적 exhaustiveness(완전성 검사)는 **논리 누락을 원천 차단**한다.  
- `Option`과 `Result`는 Rust의 “안전한 오류 처리” 철학을 대표한다.  
- 오늘 배운 내용은 이후 Day08의 `Error Handling`과 Day11의 `Concurrency`의 기반이 된다.

