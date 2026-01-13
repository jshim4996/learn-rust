// STEP 5-7, 5-8: 클로저와 이터레이터

fn main() {
    // ========================================
    // 5-7. 클로저
    // ========================================

    println!("=== 클로저 ===");

    // 기본 클로저
    let add = |a, b| a + b;
    println!("1 + 2 = {}", add(1, 2));

    // 타입 명시
    let add_explicit = |a: i32, b: i32| -> i32 { a + b };
    println!("3 + 4 = {}", add_explicit(3, 4));

    // 환경 캡처 (Fn - 불변 빌림)
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("4 == 4: {}", equal_to_x(4));

    // FnMut - 가변 빌림
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("Count: {}", count);
    };
    increment();
    increment();

    // FnOnce - 소유권 가져감
    let s = String::from("hello");
    let consume = || {
        println!("Consuming: {}", s);
        // s가 클로저 안으로 이동됨
    };
    consume();
    // consume();  // 에러: FnOnce는 한 번만 호출 가능

    // move 키워드 - 강제로 소유권 이동
    let s2 = String::from("world");
    let moved_closure = move || println!("Moved: {}", s2);
    moved_closure();
    // println!("{}", s2);  // 에러: s2가 이동됨

    // 클로저를 인자로 받는 함수
    println!("\n=== 클로저를 인자로 ===");
    let numbers = vec![1, 2, 3, 4, 5];
    let result = apply_to_all(&numbers, |x| x * 2);
    println!("Doubled: {:?}", result);

    // ========================================
    // 5-8. 이터레이터
    // ========================================

    println!("\n=== 이터레이터 ===");

    let v = vec![1, 2, 3, 4, 5];

    // iter() - 불변 참조
    print!("Elements: ");
    for val in v.iter() {
        print!("{} ", val);
    }
    println!();

    // into_iter() - 소유권 가져감
    let v2 = vec![1, 2, 3];
    let v2_iter = v2.into_iter();
    for val in v2_iter {
        print!("{} ", val);
    }
    println!();
    // v2는 더 이상 사용 불가

    // iter_mut() - 가변 참조
    let mut v3 = vec![1, 2, 3];
    for val in v3.iter_mut() {
        *val *= 10;
    }
    println!("Mutated: {:?}", v3);

    // map
    println!("\n=== map ===");
    let v4 = vec![1, 2, 3];
    let doubled: Vec<i32> = v4.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", v4);
    println!("Doubled: {:?}", doubled);

    // filter
    println!("\n=== filter ===");
    let evens: Vec<&i32> = v4.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // fold (reduce)
    println!("\n=== fold ===");
    let sum: i32 = v4.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum);

    let product: i32 = v4.iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);

    // 체이닝
    println!("\n=== 체이닝 ===");
    let result: i32 = (1..=10)
        .filter(|x| x % 2 == 0)       // 짝수만
        .map(|x| x * x)                // 제곱
        .take(3)                       // 처음 3개만
        .sum();                        // 합계
    println!("(1..=10).filter(even).map(square).take(3).sum() = {}", result);

    // find, any, all
    println!("\n=== find, any, all ===");
    let numbers = vec![1, 2, 3, 4, 5];

    let first_even = numbers.iter().find(|x| *x % 2 == 0);
    println!("First even: {:?}", first_even);

    let has_even = numbers.iter().any(|x| x % 2 == 0);
    println!("Has even: {}", has_even);

    let all_positive = numbers.iter().all(|x| *x > 0);
    println!("All positive: {}", all_positive);

    // enumerate
    println!("\n=== enumerate ===");
    for (i, val) in numbers.iter().enumerate() {
        println!("Index {}: {}", i, val);
    }

    // zip
    println!("\n=== zip ===");
    let names = vec!["Alice", "Bob", "Charlie"];
    let scores = vec![85, 90, 78];

    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{}: {}", name, score);
    }
}

// 클로저를 인자로 받는 함수
fn apply_to_all<F>(items: &[i32], f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    items.iter().map(|&x| f(x)).collect()
}
