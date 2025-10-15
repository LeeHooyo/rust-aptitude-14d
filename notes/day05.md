# Day05 — Structs & Traits

**Objectives:**  
- 구조체와 메서드로 데이터와 행위를 함께 정의한다.  
- Trait을 통해 공통 인터페이스를 설계하고, 다형성을 이해한다.  
- `derive`와 `impl`을 활용해 구조체의 기본 동작을 자동/수동으로 구현한다.

**Concepts:** `struct`, `impl`, `trait`, `derive`, `self`, `T: Trait`

---

## Commands
```bash
cargo run -p day05_structs
cargo test -p day05_structs
```

## Notes
- `struct`는 데이터, `impl`은 행동.  
- `self`, `&self`, `&mut self` → 소유권/불변/가변 권한 구분.  
- `trait`는 “이 타입은 이런 기능을 제공해야 한다”는 계약서.  
- `derive`는 표준 트레잇(`Debug`, `Clone`, `PartialEq` 등)을 자동 구현.  
- 제네릭 + 트레잇 바운드(`T: Trait`)는 타입 안전성과 재사용성을 동시에 달성.

## Errors & Fixes
- ❌ **missing trait bound `T: Debug`**  
  → 원인: `println!("{:?}", item)` 사용 시 Debug 미구현.  
  → 해결: `#[derive(Debug)]` 또는 `impl Debug for T` 추가.

- ❌ **cannot borrow as mutable**  
  → 원인: `self`가 가변 참조가 아닐 때 필드 변경 시도.  
  → 해결: 메서드 시그니처를 `&mut self`로 바꾸기.

## Reflection
- Rust의 `struct` + `trait` 설계는 “데이터 = 상태 + 행위”라는 객체지향 개념을 안전하게 표현하는 방식.  
- `impl` 블록은 클래스의 메서드처럼 작동하지만, 상속이 아닌 **합성(composition)** 으로 확장된다.  
- 트레잇은 **다형성(polymorphism)** 을 “런타임 오버헤드 없이” 제공하는 핵심 메커니즘이다.  
- 오늘 배운 `impl`, `trait`, `derive`는 Day06의 `enum` 및 `match`로 자연스럽게 이어진다.

