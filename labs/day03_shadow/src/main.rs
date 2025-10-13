fn main() {
    println!("=== Day03: Mutability & Shadowing ===");

    // 1. Mutability
    let mut count = 1;
    println!("count: {count}");
    count = count + 1; // 수정 가능
    println!("count after mutation: {count}");

    // 2. Shadowing (새 변수를 같은 이름으로 다시 선언)
    let count = "two"; // 타입까지 바뀜
    println!("count after shadowing: {count}");

    // 3. Scope (변수 생존 범위)
    {
        let scoped = "inside";
        println!("scoped: {scoped}");
    }
    // println!("{scoped}"); // scope 밖에서는 접근 불가

    // 4. UTF-8 문자열 실험
    let hello = String::from("안녕");
    // let c = hello[0]; // Rust는 직접 인덱싱 불가
    let first = hello.chars().nth(0).unwrap(); // 안전한 접근
    println!("first char: {first}");

    // 5. &str vs String
    let s: &str = "immutable slice";
    let mut s2 = String::from("mutable String");
    s2.push('!');
    println!("{s} / {s2}");
}
