# STEP 1. Rust 기초 문법

> JavaScript/TypeScript 경험자를 위한 Rust 입문 - 새로운 패러다임의 시작

---

## 학습 목표
- Rust 개발 환경 설정
- 변수, 타입, 연산자 이해
- 조건문, 반복문, 함수 작성

---

## 1-1. Rust 설치 (rustup)

### 핵심 개념

```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 설치 확인
rustc --version
cargo --version

# 업데이트
rustup update
```

### 설치되는 도구
| 도구 | 역할 |
|------|------|
| `rustc` | Rust 컴파일러 |
| `cargo` | 패키지 매니저 + 빌드 도구 |
| `rustup` | Rust 버전 관리자 |

---

## 1-2. Cargo 사용법

### 핵심 개념

```bash
cargo new my_project     # 새 프로젝트 생성
cargo build              # 컴파일
cargo run                # 컴파일 + 실행
cargo check              # 빠른 문법 체크
cargo test               # 테스트 실행
cargo doc --open         # 문서 생성
```

### Node.js와 비교
| Node.js | Rust (Cargo) |
|---------|--------------|
| `npm init` | `cargo new` |
| `npm install` | `cargo build` |
| `npm run` | `cargo run` |
| `package.json` | `Cargo.toml` |

---

## 1-3. 프로젝트 생성 (cargo new)

### 핵심 개념

```bash
cargo new hello_rust
cd hello_rust
```

### 프로젝트 구조
```
hello_rust/
├── Cargo.toml    # 프로젝트 설정 (package.json과 유사)
└── src/
    └── main.rs   # 진입점
```

### Cargo.toml
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# 여기에 외부 크레이트 추가
```

---

## 1-4. 변수 선언 (let, let mut)

### 핵심 개념

```rust
fn main() {
    // 불변 변수 (기본)
    let x = 5;
    // x = 6;  // 에러! 불변

    // 가변 변수
    let mut y = 5;
    y = 6;  // OK

    println!("x = {}, y = {}", x, y);
}
```

### JavaScript와 비교
| JavaScript | Rust | 특징 |
|------------|------|------|
| `const x = 5` | `let x = 5` | 불변 |
| `let x = 5` | `let mut x = 5` | 가변 |

### 실무 포인트
> Rust는 기본이 불변. 의도적으로 변경이 필요할 때만 `mut` 사용

---

## 1-5. 불변성 (immutability)

### 핵심 개념

```rust
fn main() {
    let x = 5;
    println!("x = {}", x);

    // x = 6;  // 컴파일 에러!
    // error: cannot assign twice to immutable variable

    let mut y = 5;
    y = 6;  // OK - mut으로 선언했으므로
}
```

### 왜 불변이 기본인가?
- 버그 방지
- 동시성 안전
- 컴파일러 최적화
- 코드 예측 가능

---

## 1-6. 기본 타입 (i32, f64, bool, char)

### 핵심 개념

```rust
fn main() {
    // 정수
    let a: i32 = 42;      // 32비트 부호 있는 정수
    let b: u32 = 42;      // 32비트 부호 없는 정수
    let c: i64 = 42;      // 64비트

    // 부동소수점
    let d: f64 = 3.14;    // 64비트 (기본)
    let e: f32 = 3.14;    // 32비트

    // 불리언
    let f: bool = true;

    // 문자 (Unicode)
    let g: char = '한';   // 4바이트 Unicode

    println!("{} {} {} {} {} {} {}", a, b, c, d, e, f, g);
}
```

### 정수 타입
| 타입 | 크기 | 범위 |
|------|------|------|
| `i8` | 8bit | -128 ~ 127 |
| `i32` | 32bit | 약 ±21억 |
| `i64` | 64bit | 매우 큼 |
| `u32` | 32bit | 0 ~ 약 42억 |
| `isize` | 아키텍처 | 포인터 크기 |

---

## 1-7. 문자열 (String, &str)

### 핵심 개념

```rust
fn main() {
    // &str - 문자열 슬라이스 (불변, 스택)
    let s1: &str = "Hello";

    // String - 힙에 할당된 가변 문자열
    let s2: String = String::from("Hello");
    let s3: String = "Hello".to_string();

    // 문자열 연결
    let mut s4 = String::from("Hello");
    s4.push_str(", World!");

    // format! 매크로
    let s5 = format!("{} {}", s1, s2);

    println!("{}", s5);
}
```

### JavaScript와 비교
| JavaScript | Rust |
|------------|------|
| `"Hello"` | `"Hello"` (&str) |
| `new String("Hello")` | `String::from("Hello")` |
| `str1 + str2` | `format!("{}{}", s1, s2)` |

---

## 1-8. 타입 추론

### 핵심 개념

```rust
fn main() {
    let x = 5;           // i32로 추론
    let y = 3.14;        // f64로 추론
    let z = true;        // bool로 추론
    let s = "hello";     // &str로 추론

    // 명시적 타입 지정
    let a: i64 = 100;
    let b = 100_i64;     // 접미사로 지정
    let c = 100u32;
}
```

---

## 1-9. 상수 (const)

### 핵심 개념

```rust
// 상수 - 컴파일 타임에 결정, 전역 가능
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;

fn main() {
    println!("Max: {}, PI: {}", MAX_POINTS, PI);
}
```

### let vs const
| `let` | `const` |
|-------|---------|
| 런타임 값 | 컴파일타임 상수 |
| 함수 내부만 | 전역 가능 |
| 타입 추론 가능 | 타입 필수 |

---

## 1-10. 섀도잉 (shadowing)

### 핵심 개념

```rust
fn main() {
    let x = 5;
    let x = x + 1;       // 새로운 x (섀도잉)
    let x = x * 2;       // 또 새로운 x

    println!("x = {}", x);  // 12

    // 타입 변경도 가능
    let spaces = "   ";
    let spaces = spaces.len();  // &str → usize
}
```

### mut과 차이
```rust
// 섀도잉 - 새 변수 생성 (타입 변경 가능)
let x = "hello";
let x = x.len();

// mut - 같은 변수 수정 (타입 변경 불가)
let mut y = "hello";
// y = y.len();  // 에러! 타입 변경 불가
```

---

## 1-11. 산술 연산자

### 핵심 개념

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("{} + {} = {}", a, b, a + b);   // 13
    println!("{} - {} = {}", a, b, a - b);   // 7
    println!("{} * {} = {}", a, b, a * b);   // 30
    println!("{} / {} = {}", a, b, a / b);   // 3 (정수 나눗셈)
    println!("{} % {} = {}", a, b, a % b);   // 1

    // 실수 나눗셈
    let c = 10.0;
    let d = 3.0;
    println!("{} / {} = {}", c, d, c / d);   // 3.333...
}
```

---

## 1-12. 비교 연산자

### 핵심 개념

```rust
fn main() {
    let a = 5;
    let b = 10;

    println!("{} == {} : {}", a, b, a == b);  // false
    println!("{} != {} : {}", a, b, a != b);  // true
    println!("{} > {} : {}", a, b, a > b);    // false
    println!("{} < {} : {}", a, b, a < b);    // true
    println!("{} >= {} : {}", a, b, a >= b);  // false
    println!("{} <= {} : {}", a, b, a <= b);  // true
}
```

---

## 1-13. 논리 연산자

### 핵심 개념

```rust
fn main() {
    let a = true;
    let b = false;

    println!("{} && {} = {}", a, b, a && b);  // false
    println!("{} || {} = {}", a, b, a || b);  // true
    println!("!{} = {}", a, !a);               // false
}
```

---

## 1-14. if / else

### 핵심 개념

```rust
fn main() {
    let number = 7;

    if number < 5 {
        println!("5보다 작음");
    } else if number < 10 {
        println!("5~9 사이");
    } else {
        println!("10 이상");
    }

    // if는 표현식 (값을 반환)
    let result = if number > 5 { "big" } else { "small" };
    println!("{}", result);
}
```

### JavaScript와 차이
- 조건에 괄호 불필요
- if는 표현식 (삼항 연산자처럼 사용 가능)

---

## 1-15. loop

### 핵심 개념

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    println!("count = {}", count);

    // loop에서 값 반환
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2;  // 20 반환
        }
    };
    println!("result = {}", result);
}
```

---

## 1-16. while

### 핵심 개념

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("발사!");
}
```

---

## 1-17. for / for in

### 핵심 개념

```rust
fn main() {
    // 배열 순회
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        println!("{}", element);
    }

    // 범위 순회
    for i in 0..5 {
        println!("i = {}", i);  // 0, 1, 2, 3, 4
    }

    // 역순
    for i in (1..4).rev() {
        println!("{}!", i);  // 3, 2, 1
    }
}
```

### JavaScript와 비교
| JavaScript | Rust |
|------------|------|
| `for (let i = 0; i < 5; i++)` | `for i in 0..5` |
| `for (const x of arr)` | `for x in arr` |

---

## 1-18. break / continue

### 핵심 개념

```rust
fn main() {
    // break
    for i in 0..10 {
        if i == 5 {
            break;
        }
        println!("{}", i);
    }

    // continue
    for i in 0..5 {
        if i == 2 {
            continue;
        }
        println!("{}", i);  // 0, 1, 3, 4
    }
}
```

---

## 1-19. 반복문에서 값 반환

### 핵심 개념

```rust
fn main() {
    let mut counter = 0;

    // loop에서 값 반환
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);  // 20

    // 레이블로 중첩 반복문 탈출
    'outer: for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                break 'outer;
            }
            println!("i={}, j={}", i, j);
        }
    }
}
```

---

## 1-20. 함수 선언 (fn)

### 핵심 개념

```rust
fn main() {
    greet("World");
    let sum = add(5, 3);
    println!("sum = {}", sum);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 암묵적 반환 (세미콜론 없음)
}
```

### JavaScript와 비교
```javascript
// JavaScript
function add(a, b) {
    return a + b;
}

// Rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 1-21. 매개변수와 반환값

### 핵심 개념

```rust
// 매개변수 타입 필수
fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// 반환값 없음 (void와 유사)
fn print_number(n: i32) {
    println!("Number: {}", n);
}

// 튜플 반환
fn swap(a: i32, b: i32) -> (i32, i32) {
    (b, a)
}

fn main() {
    let (x, y) = swap(1, 2);
    println!("x={}, y={}", x, y);  // x=2, y=1
}
```

---

## 1-22. 표현식 vs 구문

### 핵심 개념

```rust
fn main() {
    // 구문(Statement) - 값을 반환하지 않음
    let x = 5;  // 구문

    // 표현식(Expression) - 값을 반환
    let y = {
        let a = 3;
        a + 1  // 세미콜론 없음 = 표현식
    };
    println!("y = {}", y);  // 4

    // if도 표현식
    let z = if x > 3 { "big" } else { "small" };
}
```

### 핵심 규칙
- 세미콜론 있음 → 구문 (값 없음)
- 세미콜론 없음 → 표현식 (값 반환)

---

## 1-23. 반환값 (return, 암묵적 반환)

### 핵심 개념

```rust
// 암묵적 반환 (권장)
fn add(a: i32, b: i32) -> i32 {
    a + b  // 마지막 표현식이 반환값
}

// 명시적 return (조기 반환에 사용)
fn check(n: i32) -> &'static str {
    if n < 0 {
        return "negative";
    }
    if n == 0 {
        return "zero";
    }
    "positive"  // 암묵적 반환
}

fn main() {
    println!("{}", add(2, 3));
    println!("{}", check(-5));
}
```

---

## 예제 파일
- `examples/variables.rs` - 변수와 타입
- `examples/control_flow.rs` - 조건문/반복문
- `examples/functions.rs` - 함수

---

## 다음 단계
→ STEP 2: Rust 소유권 (핵심!)
