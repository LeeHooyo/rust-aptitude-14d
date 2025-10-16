// Advanced Iterators — inline commentary version

pub fn run() {
    println!("=== [Advanced Iterators] ===");

    // 1. iter() — 불변 참조로 순회 (읽기 전용)
    let v = vec![1, 2, 3, 4];
    let out_ref: Vec<i32> = v
        .iter()                             // &i32를 순회
        .filter(|&&x| x % 2 == 0)           // 각 요소를 더블 참조로 받음 (&&i32 → &x 언패킹)
        .map(|&x| x * 2)                    // 언패킹된 x를 2배
        .collect();                         // Vec<i32>로 수집
    println!("iter()  → {:?}", out_ref);
    // iter()는 borrow만 하므로 v를 그대로 쓸 수 있다 (소유권 이동 X)

    // 2. iter_mut() — 가변 참조 순회 (제자리 수정)
    let mut v2 = vec![10, 20, 30];
    v2.iter_mut().for_each(|x| *x += 1);    // 각 요소를 &mut i32로 받아 1 증가
    println!("iter_mut() → {:?}", v2);

    // 3. into_iter() — 소유권 이동 (값 자체 소비)
    let v3 = vec!["a".to_string(), "bb".into(), "ccc".into()];
    let lengths: Vec<usize> = v3
        .into_iter()                        // String 값을 move로 꺼냄
        .map(|s| s.len())                   // 이제 s는 String (소유권 있음)
        .collect();
    println!("into_iter() → {:?}", lengths);
    // println!("{:?}", v3); // 이미 move됨 → 컴파일 에러

    fnmut_counter_demo();
    fnonce_move_demo();
    pattern_demo();
    fold_result_demo();
}

// FnMut: 외부 변수 변경을 허용하는 클로저
pub fn fnmut_counter_demo() {
    let v = vec![1, 2, 3, 4, 5];
    let mut count = 0;

    let evens: Vec<_> = v
        .iter()
        .filter(|&&x| {
            // 외부 변수 count를 캡처하고 수정 → FnMut
            if x % 2 == 0 {
                count += 1;
                true
            } else {
                false
            }
        })
        .cloned()                            // &i32 → i32 복제
        .collect();

    println!("evens={:?}, counted={}", evens, count);
}

// FnOnce: move 캡처, 한 번만 호출 가능
pub fn fnonce_move_demo() {
    let tokens = vec!["a".to_string(), "b".into()];
    let big = String::from("BIG");

    // move 키워드로 big을 소유권 채로 캡처
    let consume = move |s: String| format!("{}:{}", big, s);

    // into_iter(): String을 move로 받음
    let out: Vec<_> = tokens.into_iter().map(consume).collect();
    println!("FnOnce out = {:?}", out);
    // consume은 move 클로저이므로 다시 호출 불가
}

// 패턴 언패킹 vs 역참조
pub fn pattern_demo() {
    let nums = vec![1, 2, 3];

    // iter() → &i32, 패턴 언패킹으로 바로 값 사용
    let a: Vec<_> = nums.iter().map(|&x| x + 1).collect();
    // iter() → &i32, 역참조로 접근 (*x)
    let b: Vec<_> = nums.iter().map(|x| *x + 1).collect();

    println!("a={:?}, b={:?}", a, b);

    // into_iter(): 소유권 이동, mut 바인딩으로 내부 변경 가능
    let c: Vec<_> = vec![10, 20, 30]
        .into_iter()
        .map(|mut x| { x += 1; x })         // x는 i32, 직접 수정 가능
        .collect();

    println!("c={:?}", c);
}

// try_fold: 오류를 Result 타입으로 누적 처리
pub fn fold_result_demo() {
    use std::num::ParseIntError;
    
    let inputs = vec!["10", "20", "oops", "30"];

    let res: Result<i32, ParseIntError> = inputs
        .iter()                             // &str 순회
        .try_fold(0, |acc, s| {             // 누적(acc) + 현재값(s)
            let n = s.parse::<i32>()?;        // parse() 실패 시 Err 전파
            Ok(acc + n)
        });

    println!("sum result = {:?}", res);
}

// 단위 테스트 모듈
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter_map() {
        let v = vec![1, 2, 3];
        let doubled: Vec<_> = v.iter().map(|&x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_iter_mut() {
        let mut v = vec![1, 2, 3];
        v.iter_mut().for_each(|x| *x += 1);
        assert_eq!(v, vec![2, 3, 4]);
    }

    #[test]
    fn test_into_iter() {
        let v = vec![String::from("a"), String::from("bb")];
        let lengths: Vec<usize> = v.into_iter().map(|s| s.len()).collect();
        assert_eq!(lengths, vec![1, 2]);
    }

    #[test]
    fn test_fnmut_counter_demo() {
        let v = vec![1, 2, 3, 4, 5];
        let mut count = 0;
        let evens: Vec<_> = v
            .iter()
            .filter(|&&x| {
                if x % 2 == 0 {
                    count += 1;
                    true
                } else {
                    false
                }
            })
            .cloned()
            .collect();
        assert_eq!(evens, vec![2, 4]);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_fnonce_move_demo() {
        let tokens = vec!["a".to_string(), "b".into()];
        let big = String::from("BIG");
        let consume = move |s: String| format!("{}:{}", big, s);
        let out: Vec<_> = tokens.into_iter().map(consume).collect();
        assert_eq!(out, vec!["BIG:a", "BIG:b"]);
    }

    #[test]
    fn test_pattern_demo() {
        let nums = vec![1, 2, 3];
        let a: Vec<_> = nums.iter().map(|&x| x + 1).collect();
        let b: Vec<_> = nums.iter().map(|x| *x + 1).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn test_fold_result_demo() {
        use std::num::ParseIntError;
        
        let inputs = vec!["10", "20", "oops", "30"];
        let res: Result<i32, ParseIntError> = inputs
            .iter()
            .try_fold(0, |acc, s| {
                let n = s.parse::<i32>()?;
                Ok(acc + n)
        });
        assert!(res.is_err());
    }
}

