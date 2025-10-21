# Day10 — File & IO Handling

**Objectives:**  
- `std::fs`, `std::io::{Read, Write, BufRead, BufReader, BufWriter}` 기본 사용  
- 텍스트 파일 읽기/쓰기 흐름 이해 (`create` → `write_all` / `open` → `read_to_string`)  
- 버퍼링 IO 동작과 성능 이점 이해 (`BufReader/BufWriter`, `flush`)  
- `Result`와 `?`를 통한 에러 전파 패턴 숙달  
- 미니 프로젝트: 키워드가 포함된 줄만 필터링해 새 파일로 저장

**Concepts:** `File`, `OpenOptions`, `Read`, `Write`, `BufRead::lines`, `BufReader`, `BufWriter`, `Result<T, E>`, `?`, `RAII`, `line.contains()`

---

## Commands
```bash
# 프로젝트/워크스페이스
cd ~/Projects/rust-aptitude-14d
cargo new labs/day10_io --vcs none
# 루트 Cargo.toml의 [workspace].members 에 "labs/day10_io" 추가
cargo check -p day10_io

# 멀티 바이너리 구조 생성
mkdir -p labs/day10_io/src/bin

# 각 실습 실행
cargo run -p day10_io --bin 01_basic_io
cargo run -p day10_io --bin 02_buffered_io
printf "apple\nbanana\norange\napple pie\n" > input.txt
cargo run -p day10_io --bin 03_filter_cli -- input.txt apple
cat filtered.txt
```

## Notes
- **파일 수명/닫기**: Rust는 스코프 종료 시 파일 핸들이 drop되어 자동으로 닫힘(RAII). `flush()`는 버퍼 비움이 필요할 때 명시적으로 호출.  
- **버퍼 IO**: 작은 쓰기/읽기를 모아 처리하여 시스템 호출 수를 줄임 → 체감 성능 개선. 대용량 처리나 반복 IO에 유리.  
- **에러 전파**: `main() -> io::Result<()>`와 `?` 조합으로 간결하게 실패 전파. 상위에서 메시지/exit 처리 가능.  
- **라인 단위 처리**: `BufRead::lines()`는 `Iterator<Item = io::Result<String>>`. 각 `line?` 호출 시 자동 전파.  
- **표준만으로 CLI 다루기**: `std::env::args()`로 간단한 인자 처리 가능. 복잡한 옵션은 이후 `clap`으로 확장 예정(오늘은 std-only 유지).  

## Errors & Fixes
❌ `No such file or directory (os error 2)`  
원인: `File::open("...")` 대상이 존재하지 않음.  
해결: 경로 확인, 생성 순서 점검, 상대경로 대신 절대경로 사용 고려.

❌ `Permission denied (os error 13)`  
원인: 쓰기 권한 부족 혹은 접근 제한된 디렉토리.  
해결: 사용자 쓰기 가능한 디렉토리(`$HOME/Projects/...`)에서 실행.

❌ 실행이 안 됨 (`--bin` 이름 불일치)  
원인: 파일명이 곧 바이너리 이름. `--bin` 인자와 `src/bin/<name>.rs`의 `<name>` 일치 필요.  
해결: `cargo run -p day10_io --bin 02_buffered_io`처럼 정확히 지정.

❌ 긴 줄 처리 시 메모리 사용  
설명: `lines()`는 한 줄 단위로 메모리 적재하므로 스트리밍에 적합하나, 매우 긴 단일 줄은 별도 처리 필요.

## Reflection
버퍼 IO를 통해 시스템 호출 타이밍과 데이터 흐름을 명확히 이해했다.  
`?`를 통한 에러 전파로 코드의 가독성과 안정성을 동시에 확보할 수 있었고,  
파일 핸들의 생명주기와 flush 타이밍을 체득했다.  
Rust의 철학—명시성, 안전성, 성능—이 IO에서도 일관되게 드러난다.

