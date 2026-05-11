use wasm_bindgen::JsValue;

pub fn now_millis() -> String {
    js_sys::Date::now().to_string()
}

#[allow(dead_code)]
pub fn uuid() -> String {
    let ts = js_sys::Date::now() as u64;
    let rnd = (js_sys::Math::random() * 1000000.0) as u64;
    format!("{}-{}", ts, rnd)
}

pub fn js_value_to_string(v: &JsValue) -> String {
    v.as_string().unwrap_or_default()
}

pub fn js_value_to_f64(v: &JsValue) -> f64 {
    v.as_f64().unwrap_or(0.0)
}
