// STEP 3-4 ~ 3-7: 구조체

fn main() {
    // ========================================
    // 3-4, 3-5. 구조체 정의와 생성
    // ========================================

    println!("=== 구조체 ===");

    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("testuser"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {} ({})", user1.username, user1.email);

    // 구조체 업데이트 문법
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1  // 나머지는 user1에서
    };
    println!("User2: {} ({})", user2.username, user2.email);

    // 튜플 구조체
    let origin = Point(0, 0, 0);
    let color = Color(255, 0, 0);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);
    println!("Color: RGB({}, {}, {})", color.0, color.1, color.2);

    // ========================================
    // 3-6, 3-7. 메소드와 연관 함수
    // ========================================

    println!("\n=== 메소드 ===");

    let rect = Rectangle::new(30, 50);
    println!("Rectangle: {:?}", rect);
    println!("Area: {}", rect.area());
    println!("Can hold 10x40: {}", rect.can_hold(&Rectangle::new(10, 40)));

    let square = Rectangle::square(20);
    println!("Square: {:?}", square);
    println!("Square area: {}", square.area());

    // 가변 메소드
    let mut rect2 = Rectangle::new(10, 20);
    println!("Before double: {:?}", rect2);
    rect2.double();
    println!("After double: {:?}", rect2);
}

// ========================================
// 구조체 정의
// ========================================

struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64,
}

// 튜플 구조체
struct Point(i32, i32, i32);
struct Color(u8, u8, u8);

// 일반 구조체
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 블록 - 메소드와 연관 함수
impl Rectangle {
    // 연관 함수 (생성자)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // 메소드 (불변 참조)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 가변 메소드
    fn double(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}
