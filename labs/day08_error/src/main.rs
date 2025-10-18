mod advanced_error;
mod cli;

use advanced_error::{read_file, sum_numbers, sum_numbers_lenient, AppError};

fn main() -> Result<(), AppError> {
    println!("=== [Day08: Error Handling CLI] ===");

    let cfg = match cli::parse_from_env() {
        Ok(c) => c,
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(2);
        }
    };

    let input = read_file(&cfg.path)?;

    if cfg.strict {
        // 엄격 모드: 하나라도 잘못되면 Err
        match sum_numbers(&input) {
            Ok(sum) => println!("sum(strict) = {}", sum),
            Err(e) => {
                eprintln!("strict sum failed: {}", e);
                print_error_chain(&e);
                std::process::exit(1);
            }
        }
    } else {
        // 완화 모드: 실패 라인은 건너뛰고 합계를 출력
        let (sum, skipped) = sum_numbers_lenient(&input);
        println!("sum(lenient) = {}", sum);
        if skipped > 0 {
            eprintln!("note: skipped {} invalid line(s)", skipped);
        }
    }

    Ok(())
}

/// 에러 체인을 사람이 읽기 좋게 출력
fn print_error_chain(err: &dyn std::error::Error) {
    let mut cur: Option<&(dyn std::error::Error)> = Some(err);
    let mut depth = 0;
    while let Some(e) = cur {
        if depth == 0 {
            eprintln!("error: {}", e);
        } else {
            eprintln!("  caused by({depth}): {}", e);
        }
        cur = e.source();
        depth += 1;
    }
}

