# Day07 — Collections & Iterators

**Objectives:**  
- Rust 컬렉션(`Vec`, `HashMap`)의 구조와 접근 패턴 이해  
- `iter`, `iter_mut`, `into_iter`의 소유권 차이 체득  
- `map`, `filter`, `fold` 등 이터레이터 체인과 클로저 문법 익히기  
- `Fn`, `FnMut`, `FnOnce`의 캡처 전략 이해  

**Concepts:** `Vec<T>`, `iter()`, `iter_mut()`, `into_iter()`, `map`, `filter`, `fold`, `try_fold`, `Fn`, `FnMut`, `FnOnce`, `closure`

---

## Commands
```bash
cargo run -p day07_collections
cargo test -p day07_collections
```

## Notes

- **`Vec<T>`**  
  Rust의 기본 가변 컬렉션. Heap에 연속적으로 저장되며, 고정 크기 배열(`array`)보다 유연함.  
  `Vec::new()`, `vec![1,2,3]`, `push`, `pop`, `collect` 등으로 생성 및 변환 가능.  

- **`iter()` / `iter_mut()` / `into_iter()`**
  | 메서드 | 반환 타입 | 전달값 | 소유권 |
  |--------|-----------|--------|--------|
  | `.iter()` | `&T` | 불변 참조 | 유지 |
  | `.iter_mut()` | `&mut T` | 가변 참조 | 유지 |
  | `.into_iter()` | `T` | 값 이동 | 이동 |

  - `iter()` : 읽기 전용 순회. 원본 보존.  
  - `iter_mut()` : 가변 참조 순회. 내부 값 수정 가능.  
  - `into_iter()` : 각 요소의 소유권 이동. 이후 원본 사용 불가.  

- **클로저(`|x| x + 1`)**  
  익명 함수로, 외부 변수를 캡처할 수 있음.  
  - 외부 변수 읽기만 → `Fn`  
  - 외부 변수 수정 → `FnMut`  
  - 외부 변수 move → `FnOnce`

- **이터레이터 체인 (Iterator Adapters)**  
  - `.map()` : 변환  
  - `.filter()` : 조건 유지  
  - `.fold()` / `.try_fold()` : 누적 계산  
  - `.collect()` : 결과를 컬렉션으로 변환  

  이 체인들은 **지연 평가(lazy evaluation)** 로 동작.  
  `collect` 같은 “소비자” 호출 시 실제 연산 수행.  

- **패턴 언패킹**
  `|&x|` = 참조 언패킹, `|x| *x` = 역참조. 성능 차이 없음.

- **`try_fold`**
  오류(`Result`)를 반환하는 연산에 유용.  
  `?` 연산자와 함께 `Err`를 즉시 반환하며 루프 조기 종료 가능.  

## Errors & Fixes

- ❌ `type annotations needed (E0282)`  
  **원인:** `Result`의 에러 타입 `E` 추론 불가.  
  **해결:** `Ok::<T, E>(...)` 또는 변수 선언에 `Result<_, ParseIntError>` 명시.  

- ⚠️ `unused import: super::*`  
  **원인:** 테스트 모듈에서 상위 모듈 가져오지 않음.  
  **해결:** import 삭제 또는 super 모듈 함수 호출 추가.

## Reflection

- Rust의 이터레이터는 **C의 for 루프보다 훨씬 더 안전하고 함수형적**이다.  
- 모든 순회 연산이 `borrow` / `mut borrow` / `move` 로 구분되어 있음은  
  Rust의 소유권 모델이 단순 반복조차 안전하게 설계되었음을 보여준다.  
- `Fn` 계열 트레잇은 클로저와 함수형 패러다임을 연결하는 핵심 요소.  
  **즉, Rust는 “제어된 함수형 언어”다.**  
- `try_fold`는 Rust의 “안전한 에러 전파” 사상을 가장 잘 보여주는 예제.  

