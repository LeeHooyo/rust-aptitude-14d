use std::fs::File;
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("example.txt")?;
    file.write_all(b"Hello, Rust IO!\n")?;
    println!("파일 작성 완료: example.txt");

    let mut file = File::open("example.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("파일 내용:\n{}", contents);

    Ok(())
}
