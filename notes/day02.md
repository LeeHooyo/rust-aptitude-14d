# Day 02 - Ownership & Borrowing

**Objectives:**  
- Ownership, Move, Borrow, Mutable Borrow 개념 실험  
- Borrow 충돌 및 수명(Lifetime) 규칙 이해  

**Concepts:** `ownership`, `move`, `borrow`, `&T`, `&mut T`, `drop`, `scope`

---

## Commands
```bash
cargo run -p day02_ownership
cargo test -p day02_ownership
```

## Notes

- `let s2 = s1;` → **소유권 이동 (move)**, `s1`은 무효  
- `let mut s2 = s1;` → 소유권 이동 + **새 소유자가 수정 가능**  
- `&T` → **불변 참조**, 여러 개 동시 존재 가능  
- `&mut T` → **가변 참조**, 동시에 하나만 존재 가능  
- 불변 참조가 더 이상 “사용되지 않으면” **수명 종료** → *NLL (Non-Lexical Lifetimes)*  
- 스코프로 참조 수명을 명시적으로 끊으면 **충돌 해결 가능**  
- `move`는 포인터 수준 이동 **O(1)**, `clone`은 메모리 복제 **O(n)**  

---

## Errors & Fixes

### ❌ `cannot borrow as mutable because it is also borrowed as immutable`
- **원인:** 불변 참조가 아직 “사용 중”인 상태에서 `&mut` 생성  
- **해결:** 스코프 블록으로 참조 수명 끊기, 또는 불변 참조 사용 후 drop  

### ❌ `borrowed value does not live long enough`
- **원인:** 반환 값의 참조가 지역 변수보다 오래 살아남을 때 발생  
- **해결:** 함수가 소유권을 반환하거나 참조의 lifetime을 명확히 제한  

---

## Reflection

- Rust의 borrow checker는 **“GC 없는 실시간 메모리 보호 장치”**다.  
- 참조의 사용 시점까지 추적하는 **NLL** 덕분에 불필요한 에러가 줄었다.  
- **Mutability는 소유자 기준과 참조 기준 두 계층으로 분리**된다는 점이 인상적이다.  
- C에서 런타임에 터질 버그를 Rust는 **빌드 전에 모두 차단한다.**
