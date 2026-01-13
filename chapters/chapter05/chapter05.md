# STEP 5. Rust 고급 기능

> 제네릭, 트레이트, 라이프타임, 클로저, 스마트 포인터

---

## 학습 목표
- 제네릭으로 타입 추상화
- 트레이트로 공유 동작 정의
- 라이프타임으로 참조 유효성 보장
- 클로저와 함수형 프로그래밍
- 스마트 포인터 활용

---

## 5-1. 제네릭 함수

### 핵심 개념

```rust
// 제네릭 함수
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest: {}", largest(&chars));
}
```

### TypeScript 비교
```typescript
// TypeScript 제네릭
function largest<T>(list: T[]): T {
    return list.reduce((a, b) => a > b ? a : b);
}
```

---

## 5-2. 제네릭 구조체

### 핵심 개념

```rust
struct Point<T> {
    x: T,
    y: T,
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 특정 타입에만 메서드 구현
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

---

## 5-3. 트레이트 정의

### 핵심 개념

```rust
// 트레이트 정의
trait Summary {
    fn summarize(&self) -> String;

    // 기본 구현
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

struct Article {
    title: String,
    content: String,
}

// 트레이트 구현
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}", self.title, &self.content[..50])
    }
}
```

### TypeScript 비교
```typescript
// TypeScript 인터페이스
interface Summary {
    summarize(): string;
}

class Article implements Summary {
    summarize(): string {
        return `${this.title}: ${this.content.slice(0, 50)}`;
    }
}
```

---

## 5-4. 트레이트 바운드

### 핵심 개념

```rust
// 트레이트 바운드
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// impl Trait 문법 (간단한 경우)
fn notify2(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 여러 트레이트 바운드
fn notify3<T: Summary + Display>(item: &T) {
    println!("{}", item);
}

// where 절
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

---

## 5-5. 라이프타임 기초

### 핵심 개념

```rust
// 라이프타임 명시
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("long string");
    let s2 = String::from("xyz");

    let result = longest(&s1, &s2);
    println!("Longest: {}", result);
}
```

### 왜 라이프타임이 필요한가?
```rust
// 컴파일러가 반환값의 수명을 알 수 없음
// fn longest(x: &str, y: &str) -> &str {  // 에러!
//     if x.len() > y.len() { x } else { y }
// }
```

---

## 5-6. 구조체 라이프타임

### 핵심 개념

```rust
// 참조를 포함하는 구조체
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 라이프타임 생략 규칙 적용
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
}
```

---

## 5-7. 클로저

### 핵심 개념

```rust
fn main() {
    // 기본 클로저
    let add = |a, b| a + b;
    println!("1 + 2 = {}", add(1, 2));

    // 타입 명시
    let add_typed: fn(i32, i32) -> i32 = |a, b| a + b;

    // 환경 캡처
    let x = 4;
    let equal_to_x = |z| z == x;
    println!("4 == 4: {}", equal_to_x(4));

    // move 키워드
    let s = String::from("hello");
    let closure = move || println!("{}", s);
    closure();
    // println!("{}", s);  // 에러: s가 이동됨
}
```

### TypeScript 비교
```typescript
// 화살표 함수
const add = (a: number, b: number) => a + b;

// 클로저
const x = 4;
const equalToX = (z: number) => z === x;
```

---

## 5-8. 이터레이터

### 핵심 개념

```rust
fn main() {
    let v = vec![1, 2, 3];

    // 이터레이터 생성
    let iter = v.iter();

    // map, filter, collect
    let doubled: Vec<i32> = v.iter()
        .map(|x| x * 2)
        .collect();

    let evens: Vec<&i32> = v.iter()
        .filter(|x| *x % 2 == 0)
        .collect();

    // fold (reduce)
    let sum: i32 = v.iter().fold(0, |acc, x| acc + x);

    // 체이닝
    let result: i32 = (1..=100)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .take(5)
        .sum();
}
```

---

## 5-9. Box 스마트 포인터

### 핵심 개념

```rust
fn main() {
    // 힙에 값 저장
    let b = Box::new(5);
    println!("b = {}", b);

    // 재귀적 타입
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
```

---

## 5-10. Rc (참조 카운팅)

### 핵심 개념

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

---

## 5-11. RefCell (내부 가변성)

### 핵심 개념

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // 런타임에 빌림 규칙 체크
    *data.borrow_mut() += 1;

    println!("data = {:?}", data.borrow());

    // Rc + RefCell 조합
    use std::rc::Rc;

    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&shared_data);
    let b = Rc::clone(&shared_data);

    a.borrow_mut().push(4);
    b.borrow_mut().push(5);

    println!("shared_data = {:?}", shared_data.borrow());
}
```

---

## 예제 파일
- `examples/generics.rs` - 제네릭 예제
- `examples/traits.rs` - 트레이트 예제
- `examples/lifetimes.rs` - 라이프타임 예제
- `examples/closures.rs` - 클로저와 이터레이터 예제
- `examples/smart_pointers.rs` - 스마트 포인터 예제

---

## 다음 단계
→ STEP 6: Rust 비동기 프로그래밍
