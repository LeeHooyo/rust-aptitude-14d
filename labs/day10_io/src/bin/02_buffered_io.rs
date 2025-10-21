use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

fn main() -> io::Result<()> {
    // 쓰기: 버퍼 사용
    let file = File::create("buffered.txt")?;
    let mut writer = BufWriter::new(file);
    for i in 1..=5 {
        writeln!(writer, "Line {}", i)?;
    }
    writer.flush()?; // 버퍼 비우기
    println!("buffered.txt 파일 작성 완료");

    // 읽기: 버퍼 사용
    let file = File::open("buffered.txt")?;
    let reader = BufReader::new(file);
    println!("파일 내용:");
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

