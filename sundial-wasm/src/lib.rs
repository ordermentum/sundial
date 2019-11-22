use wasm_bindgen::prelude::*;
use sundial::{get_all_iter_dates};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}


#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn get_dates(rrule: &str, count: &str, until: &str) -> Option<Box<[JsValue]>> {
    let mut rrules_result_native: Vec<String> = Vec::new();
    match get_all_iter_dates(&rrule, &count, &until) {
        Ok(rrules) => rrules_result_native = rrules,
        Err(err) => return None,
    }

    // for (i, rrule) in rrules_result_native.iter().enumerate() {
    //     let js_string = cx.string(rrule);
    //     rrule_js_array.set(&mut cx, i as u32, js_string).unwrap();
    // }
    // Ok(rrule_js_array)
    Some(vec![JsValue::NULL].into_boxed_slice())
}
