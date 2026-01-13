// STEP 2-10 ~ 2-11: 슬라이스

fn main() {
    // ========================================
    // 2-10. 문자열 슬라이스 (&str)
    // ========================================

    println!("=== 문자열 슬라이스 ===");

    let s = String::from("hello world");

    // 범위로 슬라이스
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello = '{}', world = '{}'", hello, world);

    // 간략 표현
    let hello2 = &s[..5];   // 처음부터
    let world2 = &s[6..];   // 끝까지
    let whole = &s[..];     // 전체
    println!("hello2 = '{}', world2 = '{}', whole = '{}'", hello2, world2, whole);

    // 첫 단어 찾기
    let first = first_word(&s);
    println!("First word: '{}'", first);

    // 문자열 리터럴은 &str
    let literal: &str = "hello";
    println!("Literal: {}", literal);

    // String에서 &str로
    let string = String::from("hello");
    let str_slice: &str = &string;
    println!("String as &str: {}", str_slice);

    // ========================================
    // 2-11. 배열 슬라이스
    // ========================================

    println!("\n=== 배열 슬라이스 ===");

    let arr = [1, 2, 3, 4, 5];

    let slice1 = &arr[1..3];  // [2, 3]
    let slice2 = &arr[..2];   // [1, 2]
    let slice3 = &arr[3..];   // [4, 5]

    println!("arr[1..3] = {:?}", slice1);
    println!("arr[..2] = {:?}", slice2);
    println!("arr[3..] = {:?}", slice3);

    // 함수에서 슬라이스 사용
    let sum = sum_slice(&arr);
    println!("Sum of {:?} = {}", arr, sum);

    let sum_part = sum_slice(&arr[1..4]);
    println!("Sum of [2,3,4] = {}", sum_part);

    // 가변 슬라이스
    let mut numbers = [1, 2, 3, 4, 5];
    double_values(&mut numbers[..3]);
    println!("After doubling first 3: {:?}", numbers);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    s
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in slice {
        sum += num;
    }
    sum
}

fn double_values(slice: &mut [i32]) {
    for num in slice {
        *num *= 2;
    }
}
