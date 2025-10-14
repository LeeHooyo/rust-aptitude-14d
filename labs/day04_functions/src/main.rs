// Day04 — Functions & Lifetimes
// 목적: 소유권 이동 / 참조 / 슬라이스 / 라이프타임 규칙 실습

use std::fmt::Debug;

/// 소유권을 가져가는 함수
fn take_ownership(value: String) {
    println!("[take_ownership] len = {}", value.len());
    // value는 여기서 drop
}

/// 불변 참조로 빌리는 함수
fn borrow_immutable(value: &String) -> usize {
    value.len()
}

/// 가변 참조로 빌리는 함수
fn borrow_mutable(value: &mut String) {
    value.push_str(" (mutated)");
}

/// 슬라이스에서 첫 단어를 반환
fn first_word<'a>(text: &'a str) -> &'a str {
    for (idx, byte) in text.as_bytes().iter().enumerate() {
        if *byte == b' ' {
            return &text[..idx];
        }
    }
    text
}

/// 두 문자열 중 긴 쪽 반환
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

/// 참조 대신 소유 반환
fn to_upper_owned(text: &str) -> String {
    text.to_uppercase()
}

/// 참조를 담는 구조체
#[derive(Debug)]
struct Highlight<'a> {
    part: &'a str,
}

fn debug_print<T: Debug>(value: &T) {
    println!("[debug] {:?}", value);
}

fn main() {
    println!("== Day04: Functions & Lifetimes ==");

    // 1) move
    let title = String::from("Rust Day04");
    take_ownership(title);

    // 2) borrow immutable
    let name = String::from("Ferris");
    let len = borrow_immutable(&name);
    println!("[borrow_immutable] len = {}", len);
    println!("[after immutable borrow] {}", name);

    // 3) borrow mutable
    let mut banner = String::from("Functions & Lifetimes");
    borrow_mutable(&mut banner);
    println!("[borrow_mutable] {}", banner);

    // 4) slice & lifetime
    let sentence = String::from("hello rust world");
    let word = first_word(&sentence);
    println!("[first_word] {}", word);

    let a = "ownership";
    let b = "borrowing & lifetimes";
    let longer = longest(a, b);
    println!("[longest] {}", longer);

    // 5) to_upper_owned
    let shout = to_upper_owned("borrowed becomes owned");
    println!("[to_upper_owned] {}", shout);

    // 6) 구조체 내부 참조
    let highlight = Highlight { part: first_word("slice based highlight") };
    debug_print(&highlight);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrow_immutable() {
        let s = String::from("abc");
        assert_eq!(borrow_immutable(&s), 3);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn test_longest() {
        assert_eq!(longest("a", "bc"), "bc");
    }

    #[test]
    fn test_highlight_struct() {
        let h = Highlight { part: first_word("hi there") };
        assert_eq!(h.part, "hi");
    }
}

