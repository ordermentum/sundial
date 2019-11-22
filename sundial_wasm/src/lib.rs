use sundial::get_all_iter_dates;
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct SundialOptions {
    pub rule: String,
    pub count: String,
    pub until: String,
}

#[wasm_bindgen]
pub fn get_dates(value: &JsValue) -> JsValue {
    let mut rrules_result_native = vec![];
    let options: SundialOptions = value.into_serde().unwrap();
    match get_all_iter_dates(&options.rule.to_owned(), &options.count.to_owned(), &options.until.to_owned()) {
        Ok(rrules) => rrules_result_native = rrules,
        Err(_) => return JsValue::NULL,
    }

    match JsValue::from_serde(&rrules_result_native) {
        Ok(value) => value,
        Err(_) => JsValue::NULL
    }
}
