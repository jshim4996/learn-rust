// STEP 3-8 ~ 3-12: 열거형

fn main() {
    // ========================================
    // 3-8. enum 정의
    // ========================================

    println!("=== enum ===");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    print_ip(&home);
    print_ip(&loopback);

    // ========================================
    // 3-9. match 표현식
    // ========================================

    println!("\n=== match ===");

    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::ChangeColor(255, 0, 0);
    let msg4 = Message::Quit;

    process_message(&msg1);
    process_message(&msg2);
    process_message(&msg3);
    process_message(&msg4);

    // 숫자 match
    let dice = 3;
    let result = match dice {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",  // _ 는 나머지 모든 경우
    };
    println!("Dice {}: {}", dice, result);

    // ========================================
    // 3-10. Option
    // ========================================

    println!("\n=== Option ===");

    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // match로 처리
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }

    // unwrap 계열
    let n1 = some_number.unwrap();
    let n2 = no_number.unwrap_or(0);
    let n3 = some_number.unwrap_or_default();

    println!("n1 = {}, n2 = {}, n3 = {}", n1, n2, n3);

    // map
    let doubled = some_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    // ========================================
    // 3-11. Result
    // ========================================

    println!("\n=== Result ===");

    let result1 = divide(10, 2);
    let result2 = divide(10, 0);

    match result1 {
        Ok(value) => println!("10 / 2 = {}", value),
        Err(e) => println!("Error: {}", e),
    }

    match result2 {
        Ok(value) => println!("10 / 0 = {}", value),
        Err(e) => println!("Error: {}", e),
    }

    // unwrap_or_else
    let safe_result = divide(10, 0).unwrap_or_else(|_| 0);
    println!("Safe result: {}", safe_result);

    // ========================================
    // 3-12. if let
    // ========================================

    println!("\n=== if let ===");

    let some_value = Some(3);

    // match 대신 if let
    if let Some(n) = some_value {
        println!("Value: {}", n);
    }

    // else와 함께
    let no_value: Option<i32> = None;
    if let Some(n) = no_value {
        println!("Value: {}", n);
    } else {
        println!("No value");
    }

    // enum과 함께
    let msg = Message::Move { x: 5, y: 10 };
    if let Message::Move { x, y } = msg {
        println!("Moving to ({}, {})", x, y);
    }
}

// ========================================
// 열거형 정의
// ========================================

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn print_ip(ip: &IpAddr) {
    match ip {
        IpAddr::V4(a, b, c, d) => println!("IPv4: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6: {}", addr),
    }
}

fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: RGB({}, {}, {})", r, g, b),
    }
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}
