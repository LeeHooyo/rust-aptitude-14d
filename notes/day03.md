# Day 03 — Mutability & Shadowing

**Objectives:**  
- 변수 shadowing 개념 이해  
- 가변성과 불변성의 관계 실험  
- Scope 기반 변수 재정의 패턴 학습  

**Concepts:** `mutability`, `shadowing`, `scope`, `UTF-8`, `&str`, `String`

---

## Commands
```bash
cargo run -p day03_shadow
cargo test -p day03_shadow
```

## Notes
- `let mut`은 동일 변수 수정, shadowing은 새 변수로 덮기  
- shadowing은 타입까지 바꿀 수 있다  
- `{}` 블록이 끝나면 내부 변수 drop  
- `String`은 heap 기반, `&str`은 slice 기반  
- UTF-8 인덱싱은 바이트 단위가 아니므로 금지됨  

## Errors & Fixes
- `cannot assign twice to immutable variable`  
  → `let mut`로 선언 또는 shadowing 사용  
- `the type 'String' cannot be indexed by {integer}`  
  → `.chars().nth()` 또는 slice 사용  

## Reflection
- Rust는 **변경 가능성(mutation)** 과 **재선언(shadowing)** 을 구분한다.  
- 안전한 문자열 처리와 scope 제어가 Rust 사고방식의 핵심이다.  

