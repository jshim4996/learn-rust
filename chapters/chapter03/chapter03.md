# STEP 3. Rust 구조화

> 데이터를 체계적으로 다루기 - 컬렉션, 구조체, 열거형, 모듈

---

## 학습 목표
- 컬렉션 타입 활용
- 구조체와 열거형으로 데이터 모델링
- 모듈 시스템 이해

---

## 3-1. Vec

### 핵심 개념

```rust
fn main() {
    // 생성
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    // 추가
    v.push(1);
    v.push(2);
    v.push(3);

    // 접근
    let third = &v[2];
    let third_safe = v.get(2);  // Option<&i32>

    // 순회
    for i in &v {
        println!("{}", i);
    }

    // 가변 순회
    for i in &mut v {
        *i += 10;
    }
}
```

### JavaScript 배열과 비교
| JavaScript | Rust Vec |
|------------|----------|
| `[]` | `Vec::new()` |
| `[1, 2, 3]` | `vec![1, 2, 3]` |
| `.push()` | `.push()` |
| `arr[i]` | `&v[i]` 또는 `.get(i)` |

---

## 3-2. HashMap

### 핵심 개념

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // 삽입
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);

    // 접근
    let team = String::from("Blue");
    let score = scores.get(&team);  // Option<&i32>

    // 순회
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 없을 때만 삽입
    scores.entry(String::from("Yellow")).or_insert(30);
}
```

---

## 3-3. 문자열 (String)

### 핵심 개념

```rust
fn main() {
    // 생성
    let mut s = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();

    // 추가
    s.push_str("hello");
    s.push(' ');
    s.push_str("world");

    // 연결
    let s4 = s2 + " " + &s3;
    let s5 = format!("{} {}", "hello", "world");

    // 순회
    for c in s5.chars() {
        println!("{}", c);
    }
}
```

---

## 3-4. 구조체 정의

### 핵심 개념

```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// 튜플 구조체
struct Point(i32, i32, i32);

// 유닛 구조체
struct AlwaysEqual;
```

---

## 3-5. 구조체 인스턴스 생성

### 핵심 개념

```rust
fn main() {
    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("testuser"),
        active: true,
        sign_in_count: 1,
    };

    // 구조체 업데이트 문법
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // 나머지는 user1에서
    };
}
```

---

## 3-6. 메소드 (impl)

### 핵심 개념

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 메소드 (&self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 가변 메소드 (&mut self)
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}
```

---

## 3-7. 연관 함수

### 핵심 개념

```rust
impl Rectangle {
    // 연관 함수 (self 없음) - 생성자처럼 사용
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle::new(10, 20);
    let square = Rectangle::square(10);
}
```

---

## 3-8. enum 정의

### 핵심 개념

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

---

## 3-9. match 표현식

### 핵심 개념

```rust
fn main() {
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Write(text) => println!("Text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: {},{},{}", r, g, b),
    }
}
```

---

## 3-10. Option

### 핵심 개념

```rust
fn main() {
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // match로 처리
    match some_number {
        Some(n) => println!("Number: {}", n),
        None => println!("No number"),
    }

    // unwrap 계열
    let n1 = some_number.unwrap();        // None이면 패닉
    let n2 = some_number.unwrap_or(0);    // None이면 기본값
    let n3 = some_number.unwrap_or_else(|| compute_default());
}
```

---

## 3-11. Result

### 핵심 개념

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let file = match f {
        Ok(file) => file,
        Err(error) => panic!("Failed to open: {:?}", error),
    };
}
```

---

## 3-12. if let

### 핵심 개념

```rust
fn main() {
    let some_value = Some(3);

    // match 대신 if let
    if let Some(n) = some_value {
        println!("Value: {}", n);
    }

    // else와 함께
    if let Some(n) = some_value {
        println!("Value: {}", n);
    } else {
        println!("No value");
    }
}
```

---

## 3-13. mod 키워드

### 핵심 개념

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    front_of_house::hosting::add_to_waitlist();
}
```

---

## 3-14. pub 키워드

### 핵심 개념

```rust
mod my_module {
    pub struct User {
        pub name: String,
        age: u32,  // private
    }

    impl User {
        pub fn new(name: String, age: u32) -> User {
            User { name, age }
        }
    }
}
```

---

## 3-15. use 키워드

### 핵심 개념

```rust
use std::collections::HashMap;
use std::io::{self, Read, Write};

// as로 별칭
use std::fmt::Result;
use std::io::Result as IoResult;
```

---

## 3-16. 파일로 모듈 분리

### 핵심 개념

```
src/
├── main.rs
├── lib.rs
└── garden/
    └── vegetables.rs
```

```rust
// src/main.rs
mod garden;
use garden::vegetables;

// src/garden.rs 또는 src/garden/mod.rs
pub mod vegetables;
```

---

## 예제 파일
- `examples/collections.rs` - Vec, HashMap
- `examples/structs.rs` - 구조체, impl
- `examples/enums.rs` - 열거형, match

---

## 다음 단계
→ STEP 4: Rust 에러 처리
