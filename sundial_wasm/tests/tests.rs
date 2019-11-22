    use wasm_bindgen_test::*;
    use sundial::*;
    use sundial_wasm;

    fn generate_rrule_from_json(json: &str) -> Result<RRule, SundialError> {
        let rrule = serde_json::from_str(json).unwrap();
        match validate_rrule(&rrule) {
            Ok(()) => Ok(rrule),
            Err(err) => { 
                eprintln!("Error encountered: {}", err);
                Err(err)
            }
        }
    }

    #[wasm_bindgen_test]
    fn sundial_works() {
        let rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000;TZID=Australia/Brisbane").unwrap();

        assert_eq!(
            vec![
                "2019-04-03T09:01:00+10:00".to_owned(),
                "2019-04-10T09:01:00+10:00".to_owned(),
                "2019-04-17T09:01:00+10:00".to_owned(),
                "2019-04-24T09:01:00+10:00".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        );
    }

    #[wasm_bindgen_test]
    fn sundial_wasm_works() {
        let rrule_result = sundial_wasm::get_dates();

        assert_eq!(
            vec![
                "2019-04-03T09:01:00+10:00".to_owned(),
                "2019-04-10T09:01:00+10:00".to_owned(),
                "2019-04-17T09:01:00+10:00".to_owned(),
                "2019-04-24T09:01:00+10:00".to_owned()
            ],
            rrule_result
        );
    }
