use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

/// 애플리케이션 전역 에러 타입 (std 전용)
#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    ParseInt(ParseIntError),
    EmptyInput,
    /// 간단한 context (anyhow 없이)
    Context {
        msg: &'static str,
        source: Box<dyn Error + Send + Sync>,
    },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "io error: {}", e),
            AppError::ParseInt(e) => write!(f, "parse int error: {}", e),
            AppError::EmptyInput => write!(f, "empty input"),
            AppError::Context { msg, source } => write!(f, "{}: {}", msg, source),
        }
    }
}
impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::Io(e) => Some(e),
            AppError::ParseInt(e) => Some(e),
            AppError::EmptyInput => None,
            AppError::Context { source, .. } => Some(source.as_ref()),
        }
    }
}

// ? 전파용 변환
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self { AppError::Io(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::ParseInt(e) }
}

/// Result에 context를 붙이는 간단 확장 (std만 사용)
pub trait ResultExt<T> {
    fn context(self, msg: &'static str) -> Result<T, AppError>;
}
impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn context(self, msg: &'static str) -> Result<T, AppError> {
        self.map_err(|e| AppError::Context { msg, source: Box::new(e) })
    }
}

/// 파일 읽기 (문자열)
pub fn read_file(path: &str) -> Result<String, AppError> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

/// 첫 줄 숫자 엄격 파싱
pub fn read_number_strict(path: &str) -> Result<i32, AppError> {
    let s = read_file(path)?;
    let first = s.lines().next().ok_or(AppError::EmptyInput)?;
    Ok(first.trim().parse()?)
}

/// 여러 줄에서 정수만 더함 (하나라도 깨지면 Err)
pub fn sum_numbers(input: &str) -> Result<i32, AppError> {
    input
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .try_fold(0i32, |acc, line| {
            let n: i32 = line.parse()?;
            Ok(acc + n)
        })
}

/// 완화 모드: 실패 라인은 건너뛰고 합계를 반환, 몇 줄을 스킵했는지도 반환
pub fn sum_numbers_lenient(input: &str) -> (i32, usize) {
    let mut sum = 0i32;
    let mut skipped = 0usize;
    for line in input.lines().map(str::trim).filter(|l| !l.is_empty()) {
        match line.parse::<i32>() {
            Ok(n) => sum += n,
            Err(_) => skipped += 1,
        }
    }
    (sum, skipped)
}

/// 간단한 context 데모
pub fn read_with_context(path: &str) -> Result<String, AppError> {
    let mut buf = String::new();
    File::open(path)
        .context("open failed")?
        .read_to_string(&mut buf)
        .context("read_to_string failed")?;
    if buf.is_empty() {
        return Err(AppError::EmptyInput);
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lenient_skips_invalid_lines() {
        let input = "10\nx\n20\n";
        let (sum, skipped) = sum_numbers_lenient(input);
        assert_eq!(sum, 30);
        assert_eq!(skipped, 1);
    }

    #[test]
    fn strict_fails_on_invalid() {
        let input = "10\nx\n20\n";
        let res: Result<i32, AppError> = input
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty())
            .try_fold(0i32, |acc, line| {
                let n: i32 = line.parse()?;
                Ok(acc + n)
            });
        assert!(matches!(res, Err(AppError::ParseInt(_))));
    }
}

