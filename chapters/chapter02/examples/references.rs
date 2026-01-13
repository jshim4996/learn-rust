// STEP 2-6 ~ 2-9: 참조와 빌림

fn main() {
    // ========================================
    // 2-6. 참조 (&)
    // ========================================

    println!("=== 참조 (&) ===");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 참조 전달
    println!("'{}' has length {}", s1, len);  // s1 여전히 유효

    // 여러 불변 참조 가능
    let r1 = &s1;
    let r2 = &s1;
    println!("r1 = {}, r2 = {}", r1, r2);

    // ========================================
    // 2-7. 가변 참조 (&mut)
    // ========================================

    println!("\n=== 가변 참조 (&mut) ===");

    let mut s2 = String::from("hello");
    println!("Before: {}", s2);

    change(&mut s2);
    println!("After: {}", s2);

    // 가변 참조 직접 사용
    let mut s3 = String::from("hello");
    {
        let r = &mut s3;
        r.push_str(", world");
    }
    println!("s3 = {}", s3);

    // ========================================
    // 2-8. 빌림 규칙
    // ========================================

    println!("\n=== 빌림 규칙 ===");

    let mut s = String::from("hello");

    // 여러 불변 참조 OK
    {
        let r1 = &s;
        let r2 = &s;
        println!("r1 = {}, r2 = {}", r1, r2);
    }  // r1, r2 스코프 끝

    // 이제 가변 참조 가능
    {
        let r3 = &mut s;
        r3.push_str(", world");
        println!("r3 = {}", r3);
    }

    println!("s = {}", s);

    // ========================================
    // 2-9. 댕글링 참조 방지
    // ========================================

    println!("\n=== 댕글링 참조 방지 ===");

    // 이 함수는 컴파일되지 않음:
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s  // 에러! s는 drop되므로 참조가 무효
    // }

    // 대신 소유권 반환
    let s4 = no_dangle();
    println!("no_dangle: {}", s4);

    // 참조와 함께 사용하는 패턴
    let result = find_first_word("hello world");
    println!("First word: {}", result);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s는 참조일 뿐, drop되지 않음

fn change(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 소유권 이동
}

fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    s
}
