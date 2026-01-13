# STEP 2. Rust 소유권 (핵심)

> Rust의 가장 중요한 개념 - 메모리 안전을 보장하는 핵심 메커니즘

---

## 학습 목표
- 소유권 시스템 완벽 이해
- 참조와 빌림 규칙 숙지
- 슬라이스 활용

---

## 2-1. 소유권 개념

### 핵심 개념

Rust에서 모든 값은 **소유자(Owner)**가 있음:

```rust
fn main() {
    let s1 = String::from("hello");  // s1이 소유자
    let s2 = s1;                      // 소유권이 s2로 이동

    // println!("{}", s1);  // 에러! s1은 더 이상 유효하지 않음
    println!("{}", s2);     // OK
}
```

### 왜 소유권이 필요한가?
- **가비지 컬렉터 없음**: GC 없이 메모리 안전
- **데이터 레이스 방지**: 컴파일 타임에 방지
- **메모리 누수 방지**: 스코프 끝에서 자동 해제

### JavaScript와 비교
```javascript
// JavaScript - 참조 복사 (GC가 관리)
let s1 = "hello";
let s2 = s1;  // 둘 다 같은 데이터 참조
console.log(s1, s2);  // 둘 다 유효

// Rust - 소유권 이동 (GC 없음)
let s1 = String::from("hello");
let s2 = s1;  // 소유권 이동
// s1은 무효화됨
```

---

## 2-2. 소유권 규칙

### 핵심 개념

**세 가지 규칙:**

1. 각 값은 **하나의 소유자**만 가진다
2. 한 시점에 **오직 하나의 소유자**만 존재한다
3. 소유자가 스코프를 벗어나면 **값이 해제**된다

```rust
fn main() {
    {
        let s = String::from("hello");  // s가 소유자
        // s 사용 가능
    }  // s가 스코프를 벗어남 → 메모리 해제 (drop)

    // println!("{}", s);  // 에러! s는 존재하지 않음
}
```

---

## 2-3. 이동 (Move)

### 핵심 개념

힙에 할당된 데이터는 **이동**됨:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // s1 → s2로 이동 (Move)

    // s1은 더 이상 유효하지 않음
    // println!("{}", s1);  // 에러!

    // 함수에 전달해도 이동
    takes_ownership(s2);
    // println!("{}", s2);  // 에러! 소유권이 함수로 이동됨
}

fn takes_ownership(s: String) {
    println!("{}", s);
}  // s가 drop됨
```

### 메모리 구조
```
스택:          힙:
s1 → [ptr] ──→ "hello"
     [len: 5]
     [cap: 5]

이동 후:
s1 → (무효)
s2 → [ptr] ──→ "hello"
```

---

## 2-4. 복사 (Copy)

### 핵심 개념

스택에만 있는 데이터는 **복사**됨:

```rust
fn main() {
    // 정수 - Copy 타입
    let x = 5;
    let y = x;  // 복사됨

    println!("x = {}, y = {}", x, y);  // 둘 다 유효

    // Copy 타입들
    let a: i32 = 10;     // 정수
    let b: f64 = 3.14;   // 부동소수점
    let c: bool = true;  // 불리언
    let d: char = 'A';   // 문자
    let e: (i32, i32) = (1, 2);  // Copy 타입만 포함한 튜플
}
```

### Copy vs Move
| Copy (스택) | Move (힙) |
|-------------|-----------|
| `i32`, `f64`, `bool`, `char` | `String`, `Vec` |
| 복사됨, 원본 유효 | 이동됨, 원본 무효 |
| 크기 고정 | 크기 가변 |

---

## 2-5. 스코프와 드롭

### 핵심 개념

```rust
fn main() {
    {
        let s = String::from("hello");
        println!("{}", s);
    }  // ← 여기서 s가 drop됨 (메모리 해제)

    // s는 여기서 접근 불가
}

fn create_string() -> String {
    let s = String::from("hello");
    s  // 소유권 반환
}  // s가 반환되므로 drop되지 않음
```

### Drop 트레이트
```rust
struct CustomDrop {
    name: String,
}

impl Drop for CustomDrop {
    fn drop(&mut self) {
        println!("{} dropped!", self.name);
    }
}
```

---

## 2-6. 참조 (&)

### 핵심 개념

소유권 이동 없이 값을 빌려옴:

```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);  // 참조 전달

    println!("'{}' has length {}", s1, len);  // s1 여전히 유효
}

fn calculate_length(s: &String) -> usize {
    s.len()
}  // s는 참조일 뿐, drop되지 않음
```

### 메모리 구조
```
스택:          힙:
s1 → [ptr] ──→ "hello"
&s1 → [ptr] ──┘ (s1을 가리킴)
```

---

## 2-7. 가변 참조 (&mut)

### 핵심 개념

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("{}", s);  // "hello, world"
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### 제한: 한 번에 하나의 가변 참조만
```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;  // 에러! 두 개의 가변 참조 불가

    println!("{}", r1);
}
```

---

## 2-8. 빌림 규칙

### 핵심 개념

**두 가지 규칙:**

1. **여러 불변 참조** 또는 **하나의 가변 참조** (둘 다는 안됨)
2. 참조는 항상 **유효**해야 함

```rust
fn main() {
    let mut s = String::from("hello");

    // OK: 여러 불변 참조
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    // OK: 이제 가변 참조 가능 (r1, r2 사용 끝)
    let r3 = &mut s;
    println!("{}", r3);

    // 에러: 불변 참조와 가변 참조 동시 불가
    // let r4 = &s;
    // let r5 = &mut s;  // 에러!
}
```

---

## 2-9. 댕글링 참조 방지

### 핵심 개념

Rust는 댕글링 참조를 컴파일 타임에 방지:

```rust
// 에러! 댕글링 참조 반환 시도
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s  // s는 함수 끝에서 drop됨 → 참조가 무효화
// }

// OK: 소유권 반환
fn no_dangle() -> String {
    let s = String::from("hello");
    s  // 소유권 이동
}
```

---

## 2-10. 문자열 슬라이스 (&str)

### 핵심 개념

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];   // "hello"
    let world = &s[6..11];  // "world"

    // 간략 표현
    let hello2 = &s[..5];   // 처음부터
    let world2 = &s[6..];   // 끝까지
    let whole = &s[..];     // 전체

    println!("{} {}", hello, world);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
```

### String vs &str
| `String` | `&str` |
|----------|--------|
| 힙 할당, 소유 | 빌린 참조 |
| 가변 가능 | 불변 |
| `String::from("...")` | `"..."` |

---

## 2-11. 배열 슬라이스

### 핵심 개념

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    let slice = &arr[1..3];  // [2, 3]

    println!("{:?}", slice);

    // 함수에서 슬라이스 사용
    let sum = sum_slice(&arr[..]);
    println!("sum = {}", sum);
}

fn sum_slice(slice: &[i32]) -> i32 {
    let mut sum = 0;
    for &num in slice {
        sum += num;
    }
    sum
}
```

---

## 예제 파일
- `examples/ownership.rs` - 소유권, 이동, 복사
- `examples/references.rs` - 참조와 빌림
- `examples/slices.rs` - 슬라이스

---

## 다음 단계
→ STEP 3: Rust 구조화 (컬렉션, 구조체, 열거형)
