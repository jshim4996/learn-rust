# STEP 8. Rust WebAssembly

> wasm-pack과 wasm-bindgen을 활용한 웹 어셈블리 개발

---

## 학습 목표
- WebAssembly 개념 이해
- wasm-pack으로 프로젝트 생성
- wasm-bindgen으로 JS 연동
- 실전 활용 패턴

---

## 8-1. WebAssembly란?

### 핵심 개념

WebAssembly(WASM)는:
- 브라우저에서 실행되는 저수준 바이너리 포맷
- 거의 네이티브에 가까운 성능
- C/C++, Rust 등에서 컴파일
- JavaScript와 함께 동작

### 왜 Rust + WASM?
- 메모리 안전성
- 작은 바이너리 크기
- 빠른 실행 속도
- wasm-bindgen의 강력한 JS 연동

---

## 8-2. 환경 설정

### 필수 도구 설치

```bash
# wasm-pack 설치
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# 또는 cargo로 설치
cargo install wasm-pack

# wasm32 타겟 추가
rustup target add wasm32-unknown-unknown
```

### 프로젝트 생성

```bash
# 새 프로젝트 생성
wasm-pack new my-wasm-project

# 또는 기존 프로젝트에서
cargo new --lib my-wasm-lib
```

---

## 8-3. 기본 프로젝트 구조

### Cargo.toml

```toml
[package]
name = "my-wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "s"  # 크기 최적화
lto = true
```

### src/lib.rs

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

---

## 8-4. wasm-bindgen 기초

### 함수 노출

```rust
use wasm_bindgen::prelude::*;

// JS에서 호출 가능
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 문자열 반환
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// void 함수
#[wasm_bindgen]
pub fn log_message(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
```

### TypeScript 비교
```typescript
// 생성되는 JS 인터페이스
export function add(a: number, b: number): number;
export function greet(name: string): string;
export function log_message(msg: string): void;
```

---

## 8-5. JS 함수 호출

### 핵심 개념

```rust
use wasm_bindgen::prelude::*;

// JS 함수 선언
#[wasm_bindgen]
extern "C" {
    // console.log
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // alert
    fn alert(s: &str);

    // 커스텀 JS 함수
    #[wasm_bindgen(js_namespace = window)]
    fn customFunction(value: i32) -> i32;
}

#[wasm_bindgen]
pub fn call_js_functions() {
    log("Logged from Rust!");
    alert("Alert from Rust!");
}
```

---

## 8-6. 구조체 노출

### 핵심 개념

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    // 생성자
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    // getter
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    // setter
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    // 메서드
    pub fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
```

### JavaScript 사용
```javascript
import { Point } from './my_wasm';

const p1 = new Point(0, 0);
const p2 = new Point(3, 4);

console.log(p1.x);  // 0
console.log(p1.distance(p2));  // 5
```

---

## 8-7. web-sys 크레이트

### DOM 조작

```toml
# Cargo.toml
[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Document", "Element", "HtmlElement", "Window"] }
```

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, Window};

#[wasm_bindgen]
pub fn manipulate_dom() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    // 요소 가져오기
    let element: Element = document.get_element_by_id("myDiv").unwrap();

    // 내용 변경
    element.set_inner_html("Hello from Rust!");

    // 클래스 추가
    element.class_list().add_1("active")?;

    Ok(())
}
```

---

## 8-8. Canvas 그래픽

### 핵심 개념

```toml
# Cargo.toml features
[dependencies.web-sys]
version = "0.3"
features = [
    "CanvasRenderingContext2d",
    "Document",
    "HtmlCanvasElement",
    "Window",
]
```

```rust
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub fn draw_on_canvas(canvas_id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: HtmlCanvasElement = canvas.dyn_into()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // 사각형 그리기
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.fill_rect(10.0, 10.0, 100.0, 100.0);

    // 원 그리기
    ctx.begin_path();
    ctx.arc(200.0, 60.0, 50.0, 0.0, std::f64::consts::PI * 2.0)?;
    ctx.set_fill_style(&JsValue::from_str("blue"));
    ctx.fill();

    Ok(())
}
```

---

## 8-9. 비동기 처리

### 핵심 개념

```rust
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

#[wasm_bindgen]
pub async fn fetch_data(url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into()?;
    let json = JsFuture::from(resp.json()?).await?;

    Ok(json)
}
```

---

## 8-10. 빌드 및 사용

### 빌드

```bash
# 개발 빌드
wasm-pack build

# 릴리스 빌드
wasm-pack build --release

# npm 패키지 생성
wasm-pack build --target bundler

# 브라우저 직접 사용
wasm-pack build --target web
```

### JavaScript에서 사용

```javascript
// ES 모듈
import init, { greet, add } from './pkg/my_wasm.js';

async function main() {
    await init();

    console.log(greet("World"));  // "Hello, World!"
    console.log(add(1, 2));       // 3
}

main();
```

### HTML

```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Rust WASM</title>
</head>
<body>
    <script type="module">
        import init, { greet } from './pkg/my_wasm.js';

        async function run() {
            await init();
            document.body.textContent = greet("Rust");
        }

        run();
    </script>
</body>
</html>
```

---

## 예제 파일
- `examples/basic_wasm.rs` - WASM 기초
- `examples/dom_manipulation.rs` - DOM 조작
- `examples/canvas_graphics.rs` - Canvas 그래픽

---

## 학습 완료!
Rust 기초부터 WebAssembly까지 모든 과정을 마쳤습니다.
실전 프로젝트를 통해 배운 내용을 적용해 보세요.
