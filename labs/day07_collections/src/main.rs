// Day07 - Collections & Iterators

mod advanced_iter;

fn main() {
    println!("=== [Vectors & Iterators Demo] ===");

    let numbers = vec![1, 2, 3, 4, 5, 6];

    // map + filter + collect
    let doubled_even: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * 2)
        .collect();

    println!("원본: {:?}", numbers);
    println!("짝수만 2배: {:?}", doubled_even);

    // fole (누적 연산)
    let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("합계: {}", sum);

    // enumerate
    for (i, val) in numbers.iter().enumerate() {
        println!("#{} → {}", i + 1, val);
    }

    // iterator chain 응용
    let sentence = "Rust is fast and fearless";
    let words: Vec<_> = sentence.split_whitespace().collect();
    let uppercase: Vec<_> = words.iter().map(|w| w.to_uppercase()).collect();

    println!("단어 목록: {:?}", words);
    println!("대문자 변환: {:?}\n", uppercase);

    advanced_iter::run();
}
