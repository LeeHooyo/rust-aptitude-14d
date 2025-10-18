# Day08 — Error Handling

**Objectives:**  
- Rust의 `Result<T, E>`와 `Option<T>`를 통한 안전한 오류 처리  
- `unwrap`, `expect`, `?`의 차이 이해 및 에러 전파  
- `From`, `Display`, `Error` 트레잇으로 커스텀 에러 구성  
- `context`로 문맥 추가 및 에러 체인 추적  
- CLI 옵션(`--file`, `--strict`)을 통한 실제 에러 정책 구현  

**Concepts:** `Result`, `Option`, `?`, `From`, `Box<dyn Error>`, `Display`, `context`, `enum`, `match`, `unwrap_or_else`

---

## Commands
```bash
cargo run -p day08_error -- --file labs/day08_error/data_ok.txt
cargo run -p day08_error -- --file labs/day08_error/data_mix.txt --strict
cargo test -p day08_error
```

## Notes

- **에러는 값이다 (Errors as Data)**  
  Rust에서는 예외(exception)가 없고, 모든 오류는 `Result<T, E>`로 표현된다.  
  → 함수형의 *모나딕 흐름*을 차용한 설계.

- **Result 구조**
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
  모든 `?` 연산자는 `Err(e)`를 만나면 즉시 상위로 전파한다.

- **Option과의 비교**
  | Type | 의미 | 전파 방식 |
  |-------|------|-----------|
  | `Option<T>` | 값이 없을 수도 있음 (`Some` / `None`) | `?`로 `None` 전파 |
  | `Result<T,E>` | 실패 시 에러 정보 포함 (`Ok` / `Err`) | `?`로 `Err(E)` 전파 |

- **`From` 트레잇으로 전파 일원화**  
  `From<io::Error>` 등을 `AppError`에 구현하여 다양한 에러가 `?`로 자동 변환된다.

- **커스텀 에러 타입 (`enum AppError`)**
```rust
enum AppError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    EmptyInput,
    Context { msg: &'static str, source: Box<dyn std::error::Error + Send + Sync> },
}
```

- **에러 체인 (source)**  
  표준 `Error` 트레잇의 `source()` 로 근본 원인까지 추적 가능.  
  → `print_error_chain()`으로 사람이 읽기 좋게 출력.

- **CLI Error 정책**
  - `--strict`: 하나라도 실패하면 즉시 `Err`  
  - 기본 모드: 실패 라인을 건너뛰고 합계 + 스킵 개수 출력

- **컨텍스트(context)** — std만으로도 간단히 가능
```rust
File::open(path).context("open failed")?.read_to_string(&mut buf).context("read_to_string failed")?;
```

- **테스트 시나리오**
  - 정상 입력(`data_ok.txt`) → 60  
  - 혼합 입력(`data_mix.txt`)  
    - 완화 모드: 40, skipped 1  
    - 엄격 모드: parse int error

## Errors & Fixes

- ❌ `type annotations needed (E0282)`  
  원인: `Result`의 에러 타입 `E` 추론 불가  
  해결: 변수 선언에 `Result<i32, ParseIntError>` 명시하거나 `Ok::<i32, ParseIntError>(...)` 사용

- ⚠️ `unused import` 경고  
  원인: 테스트 모듈에서 사용하지 않는 `use`  
  해결: 제거하거나 실제로 사용하도록 수정

## Reflection

- Rust의 에러 시스템은 **“안전한 실패”** 를 언어 차원에서 보장한다.  
- `?`는 문법 설탕이 아니라 **명시적 제어 흐름**이다.  
- Exception 대신 `Result`를 반환하는 패턴은 신뢰성과 테스트 용이성을 동시에 올린다.  
- 간단한 `context` 만으로도 **원인→맥락**을 잇는 에러 메시지를 설계할 수 있다.  
- CLI 예제로 **실패 정책**을 설계하는 감각을 체득했다.

