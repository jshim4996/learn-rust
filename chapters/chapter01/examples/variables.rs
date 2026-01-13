// STEP 1-4 ~ 1-10: 변수와 타입

fn main() {
    // ========================================
    // 1-4. 변수 선언 (let, let mut)
    // ========================================

    // 불변 변수 (기본)
    let x = 5;
    println!("x = {}", x);
    // x = 6;  // 에러! cannot assign twice to immutable variable

    // 가변 변수
    let mut y = 5;
    println!("y before = {}", y);
    y = 6;
    println!("y after = {}", y);

    // ========================================
    // 1-5. 불변성
    // ========================================

    let immutable = 100;
    // immutable = 200;  // 컴파일 에러!

    let mut mutable = 100;
    mutable = 200;  // OK
    println!("mutable = {}", mutable);

    // ========================================
    // 1-6. 기본 타입
    // ========================================

    // 정수
    let int8: i8 = 127;
    let int32: i32 = 2_147_483_647;  // 언더스코어로 가독성
    let uint32: u32 = 4_294_967_295;

    // 부동소수점
    let float64: f64 = 3.14159;
    let float32: f32 = 2.71828;

    // 불리언
    let is_active: bool = true;
    let is_done = false;  // 타입 추론

    // 문자 (4바이트 Unicode)
    let letter: char = 'A';
    let korean: char = '한';
    let emoji: char = '🦀';

    println!("int8: {}, int32: {}, uint32: {}", int8, int32, uint32);
    println!("f64: {}, f32: {}", float64, float32);
    println!("bool: {}, {}", is_active, is_done);
    println!("char: {}, {}, {}", letter, korean, emoji);

    // ========================================
    // 1-7. 문자열
    // ========================================

    // &str - 문자열 슬라이스 (불변, 스택)
    let str_slice: &str = "Hello, Rust!";

    // String - 힙에 할당된 가변 문자열
    let string1: String = String::from("Hello");
    let string2: String = "World".to_string();

    // 문자열 연결
    let mut greeting = String::from("Hello");
    greeting.push_str(", World!");
    println!("{}", greeting);

    // format! 매크로
    let formatted = format!("{} - {}", string1, string2);
    println!("{}", formatted);

    // 문자열 길이
    println!("Length: {}", str_slice.len());

    // ========================================
    // 1-8. 타입 추론
    // ========================================

    let inferred_int = 42;        // i32로 추론
    let inferred_float = 3.14;    // f64로 추론
    let inferred_bool = true;     // bool로 추론

    // 접미사로 타입 지정
    let explicit_i64 = 100_i64;
    let explicit_u8 = 255u8;
    let explicit_f32 = 1.5f32;

    println!("i64: {}, u8: {}, f32: {}", explicit_i64, explicit_u8, explicit_f32);

    // ========================================
    // 1-9. 상수
    // ========================================

    const MAX_POINTS: u32 = 100_000;
    const PI: f64 = 3.14159265358979;

    println!("MAX: {}, PI: {}", MAX_POINTS, PI);

    // ========================================
    // 1-10. 섀도잉
    // ========================================

    let shadow = 5;
    println!("shadow 1: {}", shadow);

    let shadow = shadow + 1;  // 새로운 변수
    println!("shadow 2: {}", shadow);

    let shadow = shadow * 2;
    println!("shadow 3: {}", shadow);

    // 타입 변경도 가능
    let spaces = "   ";
    println!("spaces (str): '{}'", spaces);

    let spaces = spaces.len();
    println!("spaces (len): {}", spaces);
}
