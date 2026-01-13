// STEP 2-1 ~ 2-5: 소유권 기본

fn main() {
    // ========================================
    // 2-1. 소유권 개념
    // ========================================

    println!("=== 소유권 개념 ===");

    let s1 = String::from("hello");
    println!("s1 = {}", s1);

    let s2 = s1;  // 소유권 이동 (Move)
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1);  // 에러! s1은 더 이상 유효하지 않음

    // ========================================
    // 2-2. 소유권 규칙
    // ========================================

    println!("\n=== 소유권 규칙 ===");

    {
        let s = String::from("scoped string");
        println!("Inside scope: {}", s);
    }  // s가 스코프를 벗어남 → drop

    // println!("{}", s);  // 에러! s는 존재하지 않음

    // ========================================
    // 2-3. 이동 (Move)
    // ========================================

    println!("\n=== 이동 (Move) ===");

    let s3 = String::from("hello");
    takes_ownership(s3);
    // println!("{}", s3);  // 에러! 소유권이 함수로 이동됨

    let s4 = gives_ownership();
    println!("s4 (from gives_ownership) = {}", s4);

    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5);
    println!("s6 (returned) = {}", s6);
    // println!("{}", s5);  // 에러! s5는 이동됨

    // ========================================
    // 2-4. 복사 (Copy)
    // ========================================

    println!("\n=== 복사 (Copy) ===");

    // 정수 - Copy 타입
    let x = 5;
    let y = x;  // 복사됨
    println!("x = {}, y = {}", x, y);  // 둘 다 유효

    // 불리언 - Copy 타입
    let a = true;
    let b = a;
    println!("a = {}, b = {}", a, b);

    // 튜플 - Copy 타입만 포함하면 Copy
    let t1 = (1, 2, 3);
    let t2 = t1;
    println!("t1 = {:?}, t2 = {:?}", t1, t2);

    // ========================================
    // 2-5. 스코프와 드롭
    // ========================================

    println!("\n=== 스코프와 드롭 ===");

    {
        let _s = String::from("will be dropped");
        println!("Inside inner scope");
    }  // _s 드롭됨
    println!("After inner scope");

    // clone으로 명시적 복사
    let original = String::from("original");
    let cloned = original.clone();
    println!("original = {}, cloned = {}", original, cloned);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}  // s가 drop됨

fn gives_ownership() -> String {
    let s = String::from("from gives_ownership");
    s  // 소유권 반환
}

fn takes_and_gives_back(s: String) -> String {
    s  // 받은 소유권을 그대로 반환
}
