// STEP 4-1 ~ 4-5: 에러 처리

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // ========================================
    // 4-1. panic!
    // ========================================

    println!("=== panic! (주석 해제하면 패닉) ===");

    // panic!("crash and burn");

    // let v = vec![1, 2, 3];
    // v[99];  // 범위 초과 패닉

    // ========================================
    // 4-2. Result
    // ========================================

    println!("\n=== Result ===");

    let f = File::open("hello.txt");

    match &f {
        Ok(_) => println!("File opened successfully"),
        Err(e) => println!("Failed to open file: {}", e),
    }

    // 에러 종류에 따른 처리
    let f2 = File::open("hello.txt");
    match f2 {
        Ok(file) => println!("File: {:?}", file),
        Err(ref error) if error.kind() == io::ErrorKind::NotFound => {
            println!("File not found, creating...");
            // File::create("hello.txt")
        }
        Err(error) => {
            println!("Other error: {:?}", error);
        }
    }

    // ========================================
    // 4-3. unwrap / expect
    // ========================================

    println!("\n=== unwrap / expect ===");

    // unwrap_or - 기본값
    let content = std::fs::read_to_string("nonexistent.txt")
        .unwrap_or(String::from("default content"));
    println!("Content: {}", content);

    // unwrap_or_else - 클로저로 기본값
    let content2 = std::fs::read_to_string("nonexistent.txt")
        .unwrap_or_else(|e| {
            println!("Error occurred: {}", e);
            String::from("fallback")
        });
    println!("Content2: {}", content2);

    // unwrap_or_default
    let content3: String = std::fs::read_to_string("nonexistent.txt")
        .unwrap_or_default();
    println!("Content3: '{}'", content3);

    // ========================================
    // 4-4. ? 연산자
    // ========================================

    println!("\n=== ? 연산자 ===");

    match read_username() {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Failed to read username: {}", e),
    }

    match read_first_line("Cargo.toml") {
        Ok(line) => println!("First line: {}", line),
        Err(e) => println!("Failed: {}", e),
    }

    // ========================================
    // 4-5. 커스텀 에러 타입
    // ========================================

    println!("\n=== 커스텀 에러 ===");

    match validate_and_parse("42") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match validate_and_parse("") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }

    match validate_and_parse("abc") {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

// ? 연산자 사용 예제
fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_first_line(filename: &str) -> Result<String, io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().next().unwrap_or("").to_string())
}

// 커스텀 에러 타입
#[derive(Debug)]
enum ParseError {
    Empty,
    InvalidFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::Empty => write!(f, "Input is empty"),
            ParseError::InvalidFormat(s) => write!(f, "Invalid format: {}", s),
        }
    }
}

fn validate_and_parse(input: &str) -> Result<i32, ParseError> {
    if input.is_empty() {
        return Err(ParseError::Empty);
    }

    input.parse::<i32>()
        .map_err(|_| ParseError::InvalidFormat(input.to_string()))
}
