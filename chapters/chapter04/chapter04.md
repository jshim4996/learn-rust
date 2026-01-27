# STEP 4. Rust 에러 처리

> 안전한 에러 핸들링 - panic!과 Result를 활용한 에러 처리

---

## 학습 목표
- panic!과 복구 불가능한 에러 이해
- Result로 복구 가능한 에러 처리
- ? 연산자로 에러 전파

---

## 4-1. panic!

### 핵심 개념

```rust
fn main() {
    // 명시적 패닉
    panic!("crash and burn");

    // 암묵적 패닉 (배열 범위 초과 등)
    let v = vec![1, 2, 3];
    v[99];  // 패닉!
}
```

### 언제 panic 사용?
- 복구 불가능한 버그
- 프로토타입/예제 코드
- 테스트에서 실패 표시

---

## 4-2. Result

### 핵심 개념

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Failed to open file: {:?}", error);
        }
    };
}
```

### Result 정의
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

---

## 4-3. unwrap / expect

### 핵심 개념

```rust
use std::fs::File;

fn main() {
    // unwrap: Err이면 panic
    let f1 = File::open("hello.txt").unwrap();

    // expect: 커스텀 에러 메시지와 함께 panic
    let f2 = File::open("hello.txt")
        .expect("Failed to open hello.txt");

    // unwrap_or: 기본값 제공
    let content = std::fs::read_to_string("file.txt")
        .unwrap_or(String::from("default"));

    // unwrap_or_else: 클로저로 기본값
    let content2 = std::fs::read_to_string("file.txt")
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            String::new()
        });
}
```

---

## 4-4. ? 연산자

### 핵심 개념

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("username.txt")?;  // ?는 Err이면 조기 반환
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 더 간결하게
fn read_username_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("username.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 가장 간결
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("username.txt")
}
```

### ? 연산자 동작
```rust
// 이 코드와 동일
let f = match File::open("hello.txt") {
    Ok(file) => file,
    Err(e) => return Err(e),
};
```

---

## 4-5. 커스텀 에러 타입

### 핵심 개념

```rust
use std::fmt;

#[derive(Debug)]
enum MyError {
    NotFound,
    InvalidInput(String),
    IoError(std::io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Not found"),
            MyError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MyError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

// From 트레이트로 자동 변환
impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::IoError(error)
    }
}
```

### thiserror 크레이트 사용
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("Not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error")]
    IoError(#[from] std::io::Error),
}
```

---

## 4-6. 테스트 기초

### #[test] 속성

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }
}
```

### assert 매크로

```rust
#[test]
fn test_assertions() {
    // 조건 확인
    assert!(1 + 1 == 2);

    // 값 비교
    assert_eq!(4, 2 + 2);
    assert_ne!(4, 2 + 3);

    // 커스텀 메시지
    assert!(true, "This should be true");
    assert_eq!(4, 4, "Values should be equal");
}
```

### 테스트 실행

```bash
# 모든 테스트 실행
cargo test

# 특정 테스트만
cargo test test_add

# 출력 표시
cargo test -- --nocapture

# 무시된 테스트 실행
cargo test -- --ignored
```

### 테스트 속성

```rust
#[test]
#[ignore]  // 기본 실행에서 제외
fn expensive_test() {
    // 시간이 오래 걸리는 테스트
}

#[test]
#[should_panic]  // 패닉 발생 예상
fn test_panic() {
    panic!("This should panic");
}

#[test]
#[should_panic(expected = "out of range")]
fn test_specific_panic() {
    // "out of range" 메시지 포함 패닉 예상
}
```

---

## 예제 파일
- `examples/error_handling.rs` - 에러 처리 예제

---

## 다음 단계
→ STEP 5: Rust 고급 (제네릭, 트레이트, 라이프타임)
