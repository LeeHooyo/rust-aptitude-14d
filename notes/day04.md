# Day04 — Functions & Lifetimes

**Objectives:**  
- 함수 호출 시 소유권 이동/대여(빌림)의 차이를 실습으로 체화한다.  
- 참조와 슬라이스를 통해 “데이터를 복사하지 않고” 안전하게 다루는 방법을 익힌다.  
- 기본 라이프타임 규칙(elision)과 명시적 라이프타임 표기를 이해한다.

**Concepts:** `ownership`, `borrowing`, `&T / &mut T`, `slice`, `lifetime`, `elision`, `'static`

---

## Commands
```bash
cargo run -p day04_functions
cargo test -p day04_functions
```

## Notes

- 함수 인자 전달 시 **소유권 이동(move)** 과 **참조(빌림)** 을 구분해야 불필요한 clone을 줄일 수 있다.  
- `&T`(불변 참조)는 여러 개 동시 허용, `&mut T`(가변 참조)는 동시에 하나만 허용 → 데이터 레이스 방지의 핵심 규율.  
- **슬라이스(&str, &[T])** 는 “복사 없는 창(view)”이므로, 원본 데이터를 그대로 빌려쓴다.  
- **라이프타임**은 “참조가 유효한 범위”를 컴파일러가 추적하는 개념.  
  - elision(생략) 규칙: `fn f(x: &str) -> &str` 형태는 `'a` 없이도 자동 추론된다.  
  - 여러 참조 중 어느 것이 반환되는지 모호하면 `'a` 명시 필요.  
- **소유권을 반환**(`String`)하면 라이프타임 제약이 단순해진다.  
- `'static`: 프로그램 전체에서 유효한 문자열 리터럴 등.

## Errors & Fixes
- ❌ **cannot borrow ... as mutable more than once at a time**  
  → 원인: 같은 데이터에 대해 가변 참조 2개 이상 생성됨.  
  → 해결: 가변 참조의 범위를 명확히 분리하거나, 블록 스코프로 한정.

- ❌ **returns a reference to data owned by the current function**  
  → 원인: 함수 내부의 로컬 변수를 참조로 반환하려 함(댕글링 참조).  
  → 해결: 인자로 받은 참조를 반환하거나, 소유권을 통째로 반환(`String`, `Vec` 등).

- ❌ **cannot move out of ...**  
  → 원인: 참조를 통해 접근 중인데 move 시도.  
  → 해결: clone()은 진짜 필요할 때만, 가능하면 참조/슬라이스로 처리.

## Reflection
- 러스트는 **데이터 경쟁을 타입 시스템으로 차단**한다.  
  가변 참조 1개 규칙은 귀찮지만, 런타임 오류를 컴파일 타임에 막는다.  
- **라이프타임 표기 `'a`** 는 “언제까지 안전한가?”를 코드로 표현하는 철학이다.  
- **소유권은 이동(move)**, **참조는 빌림(borrow)**, **슬라이스는 복사 없는 창(view)** —  
  이 셋이 러스트 메모리 모델의 중심축이다.  
- 함수 설계 시 “무엇을 빌릴지, 무엇을 넘겨줄지”를 명시적으로 사고하는 습관이 생긴다.

