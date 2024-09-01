use wasm_bindgen::prelude::*;

// import Javascript's alert method to Rust
#[wasm_bindgen]
extern "C" {
    fn alert(text: &str);
}

// export Rust function greet to be used in JS/TS, the same function signature will be used in JS/TS
#[wasm_bindgen]
pub fn greet(text: &str) -> String {
    return format!("Hello, {}!", text);
}

#[wasm_bindgen]
pub fn greet_by_alert(text: &str) {
    alert(&format!("Hello, {}!", text));
}