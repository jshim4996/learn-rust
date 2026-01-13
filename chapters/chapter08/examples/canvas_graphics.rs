// STEP 8-8: Canvas 그래픽
// 빌드: wasm-pack build --target web
//
// Cargo.toml:
// [dependencies]
// wasm-bindgen = "0.2"
//
// [dependencies.web-sys]
// version = "0.3"
// features = [
//     "CanvasRenderingContext2d",
//     "Document",
//     "Element",
//     "HtmlCanvasElement",
//     "Window",
// ]

use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Canvas 컨텍스트 가져오기 헬퍼
fn get_canvas_context(canvas_id: &str) -> Result<web_sys::CanvasRenderingContext2d, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str(&format!("Canvas #{} not found", canvas_id)))?;

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into()?;

    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("Failed to get 2d context"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    Ok(ctx)
}

/// 캔버스 초기화 (배경색 채우기)
#[wasm_bindgen]
pub fn clear_canvas(canvas_id: &str, color: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .get_element_by_id(canvas_id)
        .ok_or_else(|| JsValue::from_str("Canvas not found"))?;

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into()?;
    let ctx = get_canvas_context(canvas_id)?;

    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    Ok(())
}

/// 사각형 그리기
#[wasm_bindgen]
pub fn draw_rect(
    canvas_id: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    fill_color: &str,
    stroke_color: Option<String>,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    ctx.set_fill_style(&JsValue::from_str(fill_color));
    ctx.fill_rect(x, y, width, height);

    if let Some(stroke) = stroke_color {
        ctx.set_stroke_style(&JsValue::from_str(&stroke));
        ctx.stroke_rect(x, y, width, height);
    }

    Ok(())
}

/// 원 그리기
#[wasm_bindgen]
pub fn draw_circle(
    canvas_id: &str,
    x: f64,
    y: f64,
    radius: f64,
    fill_color: &str,
    stroke_color: Option<String>,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    ctx.begin_path();
    ctx.arc(x, y, radius, 0.0, PI * 2.0)?;

    ctx.set_fill_style(&JsValue::from_str(fill_color));
    ctx.fill();

    if let Some(stroke) = stroke_color {
        ctx.set_stroke_style(&JsValue::from_str(&stroke));
        ctx.stroke();
    }

    Ok(())
}

/// 선 그리기
#[wasm_bindgen]
pub fn draw_line(
    canvas_id: &str,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    color: &str,
    line_width: f64,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);

    ctx.set_stroke_style(&JsValue::from_str(color));
    ctx.set_line_width(line_width);
    ctx.stroke();

    Ok(())
}

/// 텍스트 그리기
#[wasm_bindgen]
pub fn draw_text(
    canvas_id: &str,
    text: &str,
    x: f64,
    y: f64,
    font: &str,
    color: &str,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    ctx.set_font(font);
    ctx.set_fill_style(&JsValue::from_str(color));
    ctx.fill_text(text, x, y)?;

    Ok(())
}

/// 삼각형 그리기
#[wasm_bindgen]
pub fn draw_triangle(
    canvas_id: &str,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    x3: f64,
    y3: f64,
    fill_color: &str,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    ctx.begin_path();
    ctx.move_to(x1, y1);
    ctx.line_to(x2, y2);
    ctx.line_to(x3, y3);
    ctx.close_path();

    ctx.set_fill_style(&JsValue::from_str(fill_color));
    ctx.fill();

    Ok(())
}

/// 그라데이션 사각형 그리기
#[wasm_bindgen]
pub fn draw_gradient_rect(
    canvas_id: &str,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    color1: &str,
    color2: &str,
) -> Result<(), JsValue> {
    let ctx = get_canvas_context(canvas_id)?;

    let gradient = ctx.create_linear_gradient(x, y, x + width, y + height);
    gradient.add_color_stop(0.0, color1)?;
    gradient.add_color_stop(1.0, color2)?;

    ctx.set_fill_style(&gradient);
    ctx.fill_rect(x, y, width, height);

    Ok(())
}

/// 데모: 다양한 도형 그리기
#[wasm_bindgen]
pub fn draw_demo(canvas_id: &str) -> Result<(), JsValue> {
    // 배경 초기화
    clear_canvas(canvas_id, "#f0f0f0")?;

    // 제목
    draw_text(canvas_id, "Rust WASM Canvas Demo", 150.0, 30.0, "bold 20px Arial", "#333")?;

    // 사각형들
    draw_rect(canvas_id, 20.0, 50.0, 100.0, 80.0, "#ff6b6b", Some("#c92a2a".to_string()))?;
    draw_rect(canvas_id, 140.0, 50.0, 100.0, 80.0, "#4ecdc4", Some("#0ca678".to_string()))?;
    draw_rect(canvas_id, 260.0, 50.0, 100.0, 80.0, "#ffe66d", Some("#fab005".to_string()))?;

    // 원들
    draw_circle(canvas_id, 70.0, 200.0, 40.0, "#7950f2", Some("#5f3dc4".to_string()))?;
    draw_circle(canvas_id, 190.0, 200.0, 40.0, "#ff922b", Some("#e8590c".to_string()))?;
    draw_circle(canvas_id, 310.0, 200.0, 40.0, "#51cf66", Some("#2f9e44".to_string()))?;

    // 삼각형들
    draw_triangle(canvas_id, 50.0, 330.0, 100.0, 260.0, 150.0, 330.0, "#339af0")?;
    draw_triangle(canvas_id, 170.0, 330.0, 220.0, 260.0, 270.0, 330.0, "#f06595")?;
    draw_triangle(canvas_id, 290.0, 330.0, 340.0, 260.0, 390.0, 330.0, "#20c997")?;

    // 그라데이션
    draw_gradient_rect(canvas_id, 20.0, 350.0, 360.0, 40.0, "#667eea", "#764ba2")?;

    // 선들
    draw_line(canvas_id, 20.0, 410.0, 380.0, 410.0, "#868e96", 2.0)?;
    draw_line(canvas_id, 20.0, 430.0, 380.0, 430.0, "#495057", 3.0)?;
    draw_line(canvas_id, 20.0, 455.0, 380.0, 455.0, "#212529", 5.0)?;

    Ok(())
}

/// 애니메이션용 프레임 상태
#[wasm_bindgen]
pub struct AnimationState {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    radius: f64,
    canvas_width: f64,
    canvas_height: f64,
}

#[wasm_bindgen]
impl AnimationState {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_width: f64, canvas_height: f64) -> AnimationState {
        AnimationState {
            x: 50.0,
            y: 50.0,
            dx: 3.0,
            dy: 2.0,
            radius: 20.0,
            canvas_width,
            canvas_height,
        }
    }

    /// 다음 프레임 업데이트
    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;

        // 벽에 부딪히면 반사
        if self.x + self.radius > self.canvas_width || self.x - self.radius < 0.0 {
            self.dx = -self.dx;
        }
        if self.y + self.radius > self.canvas_height || self.y - self.radius < 0.0 {
            self.dy = -self.dy;
        }
    }

    /// 현재 상태로 그리기
    pub fn draw(&self, canvas_id: &str) -> Result<(), JsValue> {
        clear_canvas(canvas_id, "#1a1a2e")?;
        draw_circle(canvas_id, self.x, self.y, self.radius, "#e94560", Some("#fff".to_string()))?;
        Ok(())
    }
}
