// STEP 1-20 ~ 1-23: 함수

fn main() {
    // ========================================
    // 1-20. 함수 선언
    // ========================================

    println!("=== 함수 선언 ===");
    greet("Rust");
    say_hello();

    // ========================================
    // 1-21. 매개변수와 반환값
    // ========================================

    println!("\n=== 매개변수와 반환값 ===");

    let sum = add(5, 3);
    println!("add(5, 3) = {}", sum);

    let product = multiply(4, 7);
    println!("multiply(4, 7) = {}", product);

    // 튜플 반환
    let (a, b) = swap(10, 20);
    println!("swap(10, 20) = ({}, {})", a, b);

    // ========================================
    // 1-22. 표현식 vs 구문
    // ========================================

    println!("\n=== 표현식 vs 구문 ===");

    // 블록 표현식
    let y = {
        let x = 3;
        x + 1  // 세미콜론 없음 = 표현식, 값 반환
    };
    println!("block expression: {}", y);

    // if 표현식
    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("if expression: {}", number);

    // ========================================
    // 1-23. 반환값
    // ========================================

    println!("\n=== 반환값 ===");

    // 암묵적 반환
    println!("square(5) = {}", square(5));

    // 조기 반환
    println!("check(-5) = {}", check(-5));
    println!("check(0) = {}", check(0));
    println!("check(5) = {}", check(5));

    // 다양한 함수 호출
    println!("\ndistance(3, 4) = {}", distance(3.0, 4.0));
    println!("factorial(5) = {}", factorial(5));
    println!("fibonacci(10) = {}", fibonacci(10));
}

// ========================================
// 함수 정의들
// ========================================

// 반환값 없는 함수
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn say_hello() {
    println!("Hello!");
}

// 반환값 있는 함수
fn add(a: i32, b: i32) -> i32 {
    a + b  // 암묵적 반환
}

fn multiply(x: i32, y: i32) -> i32 {
    return x * y;  // 명시적 반환
}

// 튜플 반환
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

// 암묵적 반환 (권장)
fn square(n: i32) -> i32 {
    n * n
}

// 조기 반환
fn check(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    "positive"  // 마지막은 암묵적 반환
}

// 피타고라스 거리
fn distance(x: f64, y: f64) -> f64 {
    (x * x + y * y).sqrt()
}

// 팩토리얼 (재귀)
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

// 피보나치 (재귀)
fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
