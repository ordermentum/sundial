use wasm_bindgen::prelude::*;
use sundial::{get_all_iter_dates};

#[wasm_bindgen]
pub fn get_dates() -> Option<Box<[JsValue]>> {
    let rrule = "FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000;TZID=Australia/Brisbane";

    let mut rrules_result_native = vec![];

    match get_all_iter_dates(&rrule, "10", "") {
        Ok(rrules) => rrules_result_native = rrules,
        Err(err) => return None,
    }

    Some(vec![JsValue::NULL].into_boxed_slice())
}
