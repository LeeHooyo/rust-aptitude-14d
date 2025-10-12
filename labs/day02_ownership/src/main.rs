// Day 02 - Ownership & Borrowing 실험

fn main() {
    println!("=== [1] Ownership 이동 (Move) ===");
    ownership_move();

    println!("\n=== [2] Borrowing (참조) ===");
    borrowing_basic();

    println!("\n=== [3] Mutable Borrowing (가변 참조) ===");
    mutable_borrow();

    println!("\n=== [4] Borrow 충돌 재현 ===");
    borrow_conflict_demo();

    println!("\n=== [5] 스코프 끊어서 해결 ===");
    borrow_conflict_fixed();

    println!("\n=== [6] 한 번의 하나의 &mut ===");
    one_mut_at_a_time();

    println!("\n=== [7] move vs clone 시간 비교 ===");
    move_vs_clone();
}

// 1. Ownership 이동
fn ownership_move() {
    let s1 = String::from("Rust");
    let s2 = s1; // 소유권이 s2로 이동 (Move)
    // println!("{}", s1); // 에러! s1은 더 이상 유효하지 않음
    println!("s2 owns the data: {}", s2);
}

// 2. Borrowing (불변 참조)
fn borrowing_basic() {
    let s = String::from("Ownership");
    print_length(&s); // &s는 소유권을 넘기지 않고 참조만 함
    println!("s is still valid here: {}", s); // 여전히 사용 가능
}

fn print_length(s_ref: &String) {
    println!("length of '{}' is {}", s_ref, s_ref.len());
}

// 3. Mutable Borrowing (가변 참조)
fn mutable_borrow() {
    let mut name = String::from("Hooyo");
    change_name(&mut name); // 가변 참조로 전달
    println!("changed name: {}", name);
}

fn change_name(n: &mut String) {
    n.push_str(" Lee");
}

// 4. Borrow 충돌 재현 (의도적으로 컴파일 에러를 내보기)
fn borrow_conflict_demo() {
    let mut text = String::from("blockchain");
    let r1 = &text; // 불변 참조 1
    let r2 = &text; // 불변 참조 2
    println!("{r1} / {r2}"); // 여기까지는 OK

    // 아래 줄을 잠시 주석 해제해보자:
    let r3 = &mut text;
    r3.push_str(" rocks!");

    // println!("{r1}"); // r1을 다시 사용 → 충돌 (NLL)
}

// 5. 스코프를 끊어 충돌 해결하기
fn borrow_conflict_fixed() {
    let mut text = String::from("blockchain");

    {
        // 불변 참조의 '수명'을 이 블록에서만 유지
        let r1 = &text;
        let r2 = &text;
        println!("{r1} / {r2}");
    } // r1, r2 수명 종료 → 여기서 불변 참조가 drop

    // 이제 가변 참조 허용
    let r3 = &mut text;
    r3.push_str(" rocks!");
    // r3 drop 후엔 text를 그대로 사용 가능
    println!("{text}");
}

// 6. 가변 참조는 한 번에 하나씩
fn one_mut_at_a_time() {
    let mut s = String::from("hi");
    let a = &mut s;
    a.push_str("!");
    // let b = &mut s; // 같은 시점에 또 다른 &mut 금지
    drop(a); // 명시적으로 수명 종료
    let b = &mut s; // 이제 가능
    b.push_str("!");
    println!("{s}"); // "hi!!"
}

// 7. move vs clone 차이
use std::time::Instant;

fn move_vs_clone() {
    let big = "x".repeat(2_000_000); // 2MB

    let t1 = Instant::now();
    let _moved = big; // move (0(1) 포인터 이동 수준)
    let dt_move = t1.elapsed();

    // 다시 만들어서 비교 (위에서 big은 moved되어 사용 불가)
    let big2 = "x".repeat(2_000_000);

    let t2 = Instant::now();
    let _cloned = big2.clone(); // clone (0(n) 데이터 복제)
    let dt_clone = t2.elapsed();

    println!("move: {:?} | clone: {:?}", dt_move, dt_clone);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut_change() {
        let mut s = String::from("Hooyo");
        change_name(&mut s);
        assert_eq!(s, "Hooyo Lee");
    }

    #[test]
    fn test_scope_fix() {
        let mut text = String::from("abc");
        {
            let r1 = &text;
            let r2 = &text;
            assert_eq!(r1.len(), 3);
            assert_eq!(r2.len(), 3);
        }
        let r3 = &mut text;
        r3.push('d');
        assert_eq!(text, "abcd");
    }
}
