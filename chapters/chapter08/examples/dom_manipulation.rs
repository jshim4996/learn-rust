// STEP 8-7: DOM 조작
// 빌드: wasm-pack build --target web
//
// Cargo.toml:
// [dependencies]
// wasm-bindgen = "0.2"
//
// [dependencies.web-sys]
// version = "0.3"
// features = [
//     "console",
//     "Document",
//     "Element",
//     "HtmlElement",
//     "HtmlInputElement",
//     "HtmlButtonElement",
//     "Node",
//     "NodeList",
//     "Window",
//     "Event",
//     "MouseEvent",
//     "KeyboardEvent",
// ]

use wasm_bindgen::prelude::*;

// web-sys를 사용한 DOM 조작

// console 로깅 매크로
macro_rules! console_log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

/// 문서 초기화 및 기본 DOM 조작
#[wasm_bindgen]
pub fn init_dom() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document");

    console_log!("DOM initialized from Rust!");

    // body 가져오기
    let body = document.body().expect("document should have a body");

    // 새 요소 생성
    let div = document.create_element("div")?;
    div.set_id("rust-container");
    div.set_class_name("container");
    div.set_inner_html("<h1>Hello from Rust WASM!</h1>");

    body.append_child(&div)?;

    Ok(())
}

/// ID로 요소 내용 변경
#[wasm_bindgen]
pub fn set_element_content(id: &str, content: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            element.set_inner_html(content);
            console_log!("Updated element #{} with: {}", id, content);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 요소에 클래스 추가
#[wasm_bindgen]
pub fn add_class(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            element.class_list().add_1(class_name)?;
            console_log!("Added class '{}' to #{}", class_name, id);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 요소에서 클래스 제거
#[wasm_bindgen]
pub fn remove_class(id: &str, class_name: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            element.class_list().remove_1(class_name)?;
            console_log!("Removed class '{}' from #{}", class_name, id);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 클래스 토글
#[wasm_bindgen]
pub fn toggle_class(id: &str, class_name: &str) -> Result<bool, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            let result = element.class_list().toggle(class_name)?;
            console_log!("Toggled class '{}' on #{}: {}", class_name, id, result);
            Ok(result)
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 새 요소 생성 및 추가
#[wasm_bindgen]
pub fn create_element(
    parent_id: &str,
    tag: &str,
    content: &str,
    class_name: Option<String>,
) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let parent = document
        .get_element_by_id(parent_id)
        .ok_or_else(|| JsValue::from_str(&format!("Parent #{} not found", parent_id)))?;

    let element = document.create_element(tag)?;
    element.set_inner_html(content);

    if let Some(class) = class_name {
        element.set_class_name(&class);
    }

    parent.append_child(&element)?;

    console_log!("Created <{}> in #{}", tag, parent_id);

    Ok(())
}

/// 요소 제거
#[wasm_bindgen]
pub fn remove_element(id: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            element.remove();
            console_log!("Removed element #{}", id);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 속성 설정
#[wasm_bindgen]
pub fn set_attribute(id: &str, name: &str, value: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => {
            element.set_attribute(name, value)?;
            console_log!("Set attribute {}='{}' on #{}", name, value, id);
            Ok(())
        }
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 속성 가져오기
#[wasm_bindgen]
pub fn get_attribute(id: &str, name: &str) -> Result<Option<String>, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    match document.get_element_by_id(id) {
        Some(element) => Ok(element.get_attribute(name)),
        None => Err(JsValue::from_str(&format!("Element #{} not found", id))),
    }
}

/// 선택자로 요소 개수 반환
#[wasm_bindgen]
pub fn count_elements(selector: &str) -> Result<u32, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let elements = document.query_selector_all(selector)?;
    Ok(elements.length())
}

/// 인라인 스타일 설정
#[wasm_bindgen]
pub fn set_style(id: &str, property: &str, value: &str) -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let element = document
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str(&format!("Element #{} not found", id)))?;

    // HtmlElement로 변환하여 style 접근
    let html_element = element
        .dyn_ref::<web_sys::HtmlElement>()
        .ok_or_else(|| JsValue::from_str("Not an HtmlElement"))?;

    html_element.style().set_property(property, value)?;

    console_log!("Set style {}:'{}' on #{}", property, value, id);

    Ok(())
}

use wasm_bindgen::JsCast;
