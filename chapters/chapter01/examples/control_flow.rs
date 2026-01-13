// STEP 1-11 ~ 1-19: 연산자, 조건문, 반복문

fn main() {
    // ========================================
    // 1-11. 산술 연산자
    // ========================================

    let a = 10;
    let b = 3;

    println!("=== 산술 연산자 ===");
    println!("{} + {} = {}", a, b, a + b);   // 13
    println!("{} - {} = {}", a, b, a - b);   // 7
    println!("{} * {} = {}", a, b, a * b);   // 30
    println!("{} / {} = {}", a, b, a / b);   // 3 (정수)
    println!("{} % {} = {}", a, b, a % b);   // 1

    let c = 10.0_f64;
    let d = 3.0_f64;
    println!("{} / {} = {}", c, d, c / d);   // 3.333...

    // ========================================
    // 1-12. 비교 연산자
    // ========================================

    println!("\n=== 비교 연산자 ===");
    println!("{} == {} : {}", a, b, a == b);
    println!("{} != {} : {}", a, b, a != b);
    println!("{} > {} : {}", a, b, a > b);
    println!("{} < {} : {}", a, b, a < b);
    println!("{} >= {} : {}", a, b, a >= b);
    println!("{} <= {} : {}", a, b, a <= b);

    // ========================================
    // 1-13. 논리 연산자
    // ========================================

    println!("\n=== 논리 연산자 ===");
    let t = true;
    let f = false;
    println!("{} && {} = {}", t, f, t && f);
    println!("{} || {} = {}", t, f, t || f);
    println!("!{} = {}", t, !t);

    // ========================================
    // 1-14. if / else
    // ========================================

    println!("\n=== if / else ===");
    let number = 7;

    if number < 5 {
        println!("5보다 작음");
    } else if number < 10 {
        println!("5 이상 10 미만");
    } else {
        println!("10 이상");
    }

    // if는 표현식 (값 반환)
    let condition = true;
    let result = if condition { 5 } else { 10 };
    println!("result = {}", result);

    // ========================================
    // 1-15. loop
    // ========================================

    println!("\n=== loop ===");
    let mut count = 0;

    loop {
        count += 1;
        println!("count = {}", count);
        if count >= 3 {
            break;
        }
    }

    // loop에서 값 반환
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 5 {
            break counter * 2;
        }
    };
    println!("loop result = {}", loop_result);

    // ========================================
    // 1-16. while
    // ========================================

    println!("\n=== while ===");
    let mut n = 3;

    while n != 0 {
        println!("{}!", n);
        n -= 1;
    }
    println!("발사!");

    // ========================================
    // 1-17. for / for in
    // ========================================

    println!("\n=== for ===");

    // 배열 순회
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("element: {}", element);
    }

    // 범위 순회
    println!("\nrange 0..5:");
    for i in 0..5 {
        println!("i = {}", i);
    }

    // 역순
    println!("\nreverse:");
    for i in (1..4).rev() {
        println!("{}!", i);
    }

    // enumerate
    println!("\nenumerate:");
    for (index, value) in arr.iter().enumerate() {
        println!("[{}] = {}", index, value);
    }

    // ========================================
    // 1-18. break / continue
    // ========================================

    println!("\n=== break / continue ===");

    // break
    println!("break at 5:");
    for i in 0..10 {
        if i == 5 {
            break;
        }
        println!("{}", i);
    }

    // continue
    println!("\nskip 2:");
    for i in 0..5 {
        if i == 2 {
            continue;
        }
        println!("{}", i);
    }

    // ========================================
    // 1-19. 반복문에서 값 반환
    // ========================================

    println!("\n=== 반복문에서 값 반환 ===");

    // loop에서 값 반환
    let mut cnt = 0;
    let value = loop {
        cnt += 1;
        if cnt == 10 {
            break cnt * 2;
        }
    };
    println!("returned value = {}", value);

    // 레이블로 중첩 반복문 탈출
    println!("\nlabeled loop:");
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                println!("Breaking outer at i={}, j={}", i, j);
                break 'outer;
            }
            println!("i={}, j={}", i, j);
        }
    }
}
