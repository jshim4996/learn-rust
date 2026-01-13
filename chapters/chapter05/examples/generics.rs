// STEP 5-1, 5-2: 제네릭

fn main() {
    // ========================================
    // 5-1. 제네릭 함수
    // ========================================

    println!("=== 제네릭 함수 ===");

    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    // 여러 타입으로 동작
    println!("First number: {}", first(&numbers));
    println!("First char: {}", first(&chars));

    // ========================================
    // 5-2. 제네릭 구조체
    // ========================================

    println!("\n=== 제네릭 구조체 ===");

    // 같은 타입
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("Integer point: ({}, {})", integer_point.x, integer_point.y);
    println!("Float point: ({}, {})", float_point.x, float_point.y);
    println!("Float point x: {}", float_point.x());

    // f32 전용 메서드
    let origin_point = Point { x: 3.0_f32, y: 4.0_f32 };
    println!("Distance from origin: {}", origin_point.distance_from_origin());

    // 다른 타입
    let mixed = Pair { first: 5, second: "hello" };
    println!("Pair: ({}, {})", mixed.first, mixed.second);

    // mixup
    let p1 = Pair { first: 5, second: 10.4 };
    let p2 = Pair { first: "Hello", second: 'c' };
    let p3 = p1.mixup(p2);
    println!("Mixed pair: ({}, {})", p3.first, p3.second);
}

// 제네릭 함수 - 가장 큰 값 찾기
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 제네릭 함수 - 첫 번째 요소
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

// 제네릭 구조체 - 같은 타입
struct Point<T> {
    x: T,
    y: T,
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

// 제네릭 구조체 - 다른 타입
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            first: self.first,
            second: other.second,
        }
    }
}
