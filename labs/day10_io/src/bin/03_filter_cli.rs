use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("사용법: cargo run -p day10_io --bin 03_filter_cli -- <input> <keyword>");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let keyword = &args[2];

    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    let output_file = File::create("filtered.txt")?;
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(keyword) {
            writeln!(writer, "{}", line)?;
        }
    }

    writer.flush()?;
    println!("'{}' 포함된 줄만 filtered.txt에 저장 완료", keyword);
    Ok(())
}
