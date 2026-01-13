// STEP 3-1 ~ 3-3: 컬렉션

use std::collections::HashMap;

fn main() {
    // ========================================
    // 3-1. Vec
    // ========================================

    println!("=== Vec ===");

    // 생성
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];

    // 추가
    v.push(10);
    v.push(20);
    v.push(30);
    println!("v = {:?}", v);

    // 접근
    let third = &v[2];
    println!("Third element: {}", third);

    // 안전한 접근
    match v.get(10) {
        Some(value) => println!("Value at 10: {}", value),
        None => println!("No value at index 10"),
    }

    // 순회
    print!("Elements: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // 가변 순회
    for i in &mut v {
        *i += 100;
    }
    println!("After adding 100: {:?}", v);

    // 유용한 메서드
    println!("Length: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    println!("Contains 110: {}", v.contains(&110));

    let popped = v.pop();
    println!("Popped: {:?}", popped);

    // ========================================
    // 3-2. HashMap
    // ========================================

    println!("\n=== HashMap ===");

    let mut scores = HashMap::new();

    // 삽입
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    scores.insert(String::from("Green"), 30);

    println!("scores = {:?}", scores);

    // 접근
    let team = String::from("Blue");
    if let Some(score) = scores.get(&team) {
        println!("Blue team score: {}", score);
    }

    // 순회
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // 없을 때만 삽입
    scores.entry(String::from("Yellow")).or_insert(25);
    scores.entry(String::from("Blue")).or_insert(999);  // 이미 있으므로 무시
    println!("After entry: {:?}", scores);

    // 값 수정
    let blue_score = scores.entry(String::from("Blue")).or_insert(0);
    *blue_score += 10;
    println!("After update: {:?}", scores);

    // ========================================
    // 3-3. String
    // ========================================

    println!("\n=== String ===");

    // 생성
    let mut s = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();

    // 추가
    s.push_str("Hello");
    s.push(' ');
    s.push_str("Rust");
    println!("s = {}", s);

    // 연결
    let s4 = s2 + " " + &s3;  // s2는 이동됨
    println!("s4 = {}", s4);

    // format! (소유권 이동 없음)
    let s5 = format!("{} + {} = {}", 1, 2, 3);
    println!("s5 = {}", s5);

    // 문자 순회
    print!("Chars: ");
    for c in "hello".chars() {
        print!("{} ", c);
    }
    println!();

    // 바이트 순회
    print!("Bytes: ");
    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!();
}
