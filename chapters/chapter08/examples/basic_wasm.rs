// STEP 8-3 ~ 8-6: WASM 기초
// 빌드: wasm-pack build --target web
//
// Cargo.toml:
// [lib]
// crate-type = ["cdylib", "rlib"]
//
// [dependencies]
// wasm-bindgen = "0.2"

use wasm_bindgen::prelude::*;

// ========================================
// 8-4. 기본 함수 노출
// ========================================

/// 두 숫자를 더합니다
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 두 숫자를 곱합니다
#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// 인사말을 반환합니다
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust WASM.", name)
}

/// 팩토리얼 계산 (성능 테스트용)
#[wasm_bindgen]
pub fn factorial(n: u32) -> u64 {
    if n <= 1 {
        1
    } else {
        n as u64 * factorial(n - 1)
    }
}

/// 피보나치 수열 (성능 테스트용)
#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }
}

// ========================================
// 8-5. JS 함수 호출
// ========================================

#[wasm_bindgen]
extern "C" {
    // console.log 바인딩
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // console.warn 바인딩
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);

    // alert 바인딩
    fn alert(s: &str);
}

/// 콘솔에 로그 출력
#[wasm_bindgen]
pub fn console_log(message: &str) {
    log(&format!("[Rust] {}", message));
}

/// 경고 로그 출력
#[wasm_bindgen]
pub fn console_warn(message: &str) {
    warn(&format!("[Rust Warning] {}", message));
}

/// Alert 표시
#[wasm_bindgen]
pub fn show_alert(message: &str) {
    alert(message);
}

// ========================================
// 8-6. 구조체 노출
// ========================================

/// 2D 좌표를 나타내는 Point
#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    /// 새 Point 생성
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// 원점 생성
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// x 좌표 getter
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// y 좌표 getter
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// x 좌표 setter
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// y 좌표 setter
    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// 이동
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// 다른 점과의 거리 계산
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// 원점으로부터의 거리
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 사각형
#[wasm_bindgen]
pub struct Rectangle {
    width: f64,
    height: f64,
}

#[wasm_bindgen]
impl Rectangle {
    #[wasm_bindgen(constructor)]
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    pub fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn width(&self) -> f64 {
        self.width
    }

    #[wasm_bindgen(getter)]
    pub fn height(&self) -> f64 {
        self.height
    }

    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

// ========================================
// 배열 처리
// ========================================

/// 배열의 합계 계산
#[wasm_bindgen]
pub fn sum_array(arr: &[i32]) -> i32 {
    arr.iter().sum()
}

/// 배열의 평균 계산
#[wasm_bindgen]
pub fn average_array(arr: &[f64]) -> f64 {
    if arr.is_empty() {
        0.0
    } else {
        arr.iter().sum::<f64>() / arr.len() as f64
    }
}

/// 배열을 정렬하여 반환
#[wasm_bindgen]
pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut sorted = arr.to_vec();
    sorted.sort();
    sorted
}
