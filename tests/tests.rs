#[cfg(test)]
mod tests {
    use chrono::{Datelike, TimeZone, Timelike, Utc, Weekday};
    use chrono_tz::Etc::UTC;
    use std::iter::Iterator;
    use sundial::{convert_to_rrule, validate_rrule, RRule, RuleParseError};

    fn generate_rrule_from_json(json: &str) -> Result<RRule, RuleParseError> {
        let rrule = serde_json::from_str(json).unwrap();
        match validate_rrule(&rrule) {
            Ok(()) => Ok(rrule),
            Err(err) => {
                eprintln!("Error encountered: {}", err);
                Err(RuleParseError)
            }
        }
    }

    struct RRuleTestCase<'a> {
        rrule_string: &'a str,
        expected_flat_json: &'a str,
    }

    #[test]
    fn test_we_can_parse_to_proper_json() {
        let rrule_test_cases: Vec<RRuleTestCase> = vec![
            RRuleTestCase {
                rrule_string: "FREQ=DAILY",
                expected_flat_json: r#"{"frequency":"DAILY"}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=MONTHLY;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=15,27",
                expected_flat_json: r#"{"frequency":"MONTHLY","interval":"1","byHour":["9"],"byMinute":["1"],"byMonthDay":["15","27"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=DAILY;BYHOUR=1,3",
                expected_flat_json: r#"{"frequency":"DAILY","byHour":["1","3"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=DAILY;BYHOUR=1,3",
                expected_flat_json: r#"{"frequency":"DAILY","byHour":["1","3"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=17;BYMINUTE=30;BYDAY=SU",
                expected_flat_json: r#"{"frequency":"WEEKLY","interval":"1","byHour":["17"],"byMinute":["30"],"byDay":["SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=17;BYMINUTE=30;BYDAY=SU",
                expected_flat_json: r#"{"frequency":"WEEKLY","interval":"1","byHour":["17"],"byMinute":["30"],"byDay":["SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;FREQ=FORTNIGHTLY",
                expected_flat_json: r#"{"frequency":"FORTNIGHTLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "DTSTART;TZID=Australia/Sydney:19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00 AEST","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "DTSTART;TZID=Europe/London:19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00 BST","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "DTSTART=19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "DTSTART=19970714T133000Z;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "DTSTART=19970714T133000Z;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000Z",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;UNTIL=20190422T133500;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","until":"2019-04-22 13:35:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            },
            RRuleTestCase {
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000;UNTIL=21330422T133500Z",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","until":"2133-04-22 13:35:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#,
            }
        ];

        for i in &rrule_test_cases {
            let rrule_result = convert_to_rrule(i.rrule_string).unwrap();

            assert_eq!(i.expected_flat_json, rrule_result.to_json())
        }
    }

    #[test]
    fn test_by_hour_validation_works() {
        let rrule = RRule::new_rrule(
            String::from("Australia/Perth"),
            String::from("1997-07-14 13:30:00"),
            String::from("2133-04-22 13:35:00"),
            String::from("WEEKLY"),
            String::from(""),
            String::from("1"),
            String::from(""),
            Vec::new(),
            vec!["8", "28"],
            vec!["30", "45"],
            Vec::new(),
            vec!["TU", "SU"],
            Vec::new(),
            Vec::new(),
        );
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_by_minute_validation_works() {
        let rrule = RRule::new_rrule(
            String::from("Australia/Perth"),
            String::from("1997-07-14 13:30:00"),
            String::from("2133-04-22 13:35:00"),
            String::from("WEEKLY"),
            String::from(""),
            String::from("1"),
            String::from(""),
            Vec::new(),
            vec!["8", "23"],
            vec!["30", "90"],
            Vec::new(),
            vec!["TU", "SU"],
            Vec::new(),
            Vec::new(),
        );
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_by_monthday_validation_works() {
        let rrule = RRule::new_rrule(
            String::from("Australia/Perth"),
            String::from("1997-07-14 13:30:00"),
            String::from("2133-04-22 13:35:00"),
            String::from("WEEKLY"),
            String::from(""),
            String::from("1"),
            String::from(""),
            Vec::new(),
            vec!["8", "23"],
            vec!["30", "50"],
            Vec::new(),
            vec!["TU", "SU"],
            vec!["32"],
            Vec::new(),
        );
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_tzid_validation_works() {
        let rrule = RRule::new_rrule(
            String::from("Gondwana/BigContinent"),
            String::from("1997-07-14 13:30:00"),
            String::from("2133-04-22 13:35:00"),
            String::from("WEEKLY"),
            String::from(""),
            String::from("1"),
            String::from(""),
            Vec::new(),
            vec!["8", "12"],
            vec!["30", "33"],
            Vec::new(),
            vec!["TU", "SU"],
            vec!["22"],
            Vec::new(),
        );
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_we_use_the_count_properly() {
        let rrule_result = convert_to_rrule(
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        )
        .unwrap();

        assert_eq!(27, rrule_result.get_next_iter_dates("", "").len())
    }

    #[test]
    fn test_until_params_works() {
        let rrule_result = convert_to_rrule("FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;DTSTART=20190327T133500;UNTIL=20200612T030000").unwrap();

        assert_eq!(
            vec![
                "2019-04-27T09:35:00+00:00".to_owned(),
                "2019-05-27T09:35:00+00:00".to_owned(),
                "2019-06-27T09:35:00+00:00".to_owned(),
                "2019-07-27T09:35:00+00:00".to_owned(),
                "2019-08-27T09:35:00+00:00".to_owned(),
                "2019-09-27T09:35:00+00:00".to_owned(),
                "2019-10-27T09:35:00+00:00".to_owned(),
                "2019-11-27T09:35:00+00:00".to_owned(),
                "2019-12-27T09:35:00+00:00".to_owned(),
                "2020-01-27T09:35:00+00:00".to_owned(),
                "2020-02-27T09:35:00+00:00".to_owned(),
                "2020-03-27T09:35:00+00:00".to_owned(),
                "2020-04-27T09:35:00+00:00".to_owned(),
                "2020-05-27T09:35:00+00:00".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_daily_rules_work_1() {
        let rrule_result = convert_to_rrule(
            "FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000",
        )
        .unwrap();

        assert_eq!(
            vec![
                "2019-04-03 09:01:00".to_owned(),
                "2019-04-10 09:01:00".to_owned(),
                "2019-04-17 09:01:00".to_owned(),
                "2019-04-24 09:01:00".to_owned()
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_daily_rules_work_2() {
        let rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000;TZID=Australia/Darwin").unwrap();

        assert_eq!(
            vec![
                "2019-04-03T09:01:00+09:30".to_owned(),
                "2019-04-10T09:01:00+09:30".to_owned(),
                "2019-04-17T09:01:00+09:30".to_owned(),
                "2019-04-24T09:01:00+09:30".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        );
    }

    #[test]
    fn test_daily_rules_work_3() {
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

    #[test]
    fn test_daily_rules_work_4() {
        let rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=6;INTERVAL=5;BYDAY=WE;BYHOUR=12;BYMINUTE=52;DTSTART=20190327T030000;TZID=Singapore").unwrap();

        assert_eq!(
            vec![
                "2019-05-01T12:52:00+08:00".to_owned(),
                "2019-06-05T12:52:00+08:00".to_owned(),
                "2019-07-10T12:52:00+08:00".to_owned(),
                "2019-08-14T12:52:00+08:00".to_owned(),
                "2019-09-18T12:52:00+08:00".to_owned(),
                "2019-10-23T12:52:00+08:00".to_owned(),
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        );
    }

    #[test]
    fn test_daily_rules_work_5() {
        let rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=20;INTERVAL=3;BYDAY=FR;BYMONTH=11;BYHOUR=10;BYMINUTE=1;BYSECOND=58;DTSTART=20190327T030000").unwrap();

        assert_eq!(
            vec![
                "2019-11-01 10:01:58".to_owned(),
                "2019-11-22 10:01:58".to_owned(),
                "2020-11-13 10:01:58".to_owned(),
                "2021-11-05 10:01:58".to_owned(),
                "2021-11-26 10:01:58".to_owned(),
                "2022-11-18 10:01:58".to_owned(),
                "2023-11-10 10:01:58".to_owned(),
                "2024-11-01 10:01:58".to_owned(),
                "2024-11-22 10:01:58".to_owned(),
                "2025-11-14 10:01:58".to_owned(),
                "2026-11-06 10:01:58".to_owned(),
                "2026-11-27 10:01:58".to_owned(),
                "2027-11-19 10:01:58".to_owned(),
                "2028-11-10 10:01:58".to_owned(),
                "2029-11-02 10:01:58".to_owned(),
                "2029-11-23 10:01:58".to_owned(),
                "2030-11-15 10:01:58".to_owned(),
                "2031-11-07 10:01:58".to_owned(),
                "2031-11-28 10:01:58".to_owned(),
                "2032-11-19 10:01:58".to_owned(),
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_daily_rules_work_6() {
        let rrule_result = convert_to_rrule(
            "FREQ=DAILY;COUNT=4;INTERVAL=1;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000",
        )
        .unwrap();

        assert_eq!(
            vec![
                "2019-03-27 09:01:00".to_owned(),
                "2019-03-28 09:01:00".to_owned(),
                "2019-03-29 09:01:00".to_owned(),
                "2019-03-30 09:01:00".to_owned()
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_hourly_rules_work_1() {
        let rrule_result =
            convert_to_rrule("FREQ=HOURLY;INTERVAL=3;COUNT=20;DTSTART=20190327T030000").unwrap();

        assert_eq!(
            vec![
                "2019-03-27 06:00:00".to_owned(),
                "2019-03-27 09:00:00".to_owned(),
                "2019-03-27 12:00:00".to_owned(),
                "2019-03-27 15:00:00".to_owned(),
                "2019-03-27 18:00:00".to_owned(),
                "2019-03-27 21:00:00".to_owned(),
                "2019-03-28 00:00:00".to_owned(),
                "2019-03-28 03:00:00".to_owned(),
                "2019-03-28 06:00:00".to_owned(),
                "2019-03-28 09:00:00".to_owned(),
                "2019-03-28 12:00:00".to_owned(),
                "2019-03-28 15:00:00".to_owned(),
                "2019-03-28 18:00:00".to_owned(),
                "2019-03-28 21:00:00".to_owned(),
                "2019-03-29 00:00:00".to_owned(),
                "2019-03-29 03:00:00".to_owned(),
                "2019-03-29 06:00:00".to_owned(),
                "2019-03-29 09:00:00".to_owned(),
                "2019-03-29 12:00:00".to_owned(),
                "2019-03-29 15:00:00".to_owned(),
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_hourly_rules_work_2() {
        let rrule_result = convert_to_rrule(
            "FREQ=HOURLY;INTERVAL=3;BYDAY=TU;COUNT=20;DTSTART=20190327T030000;BYHOUR=9;BYMINUTE=12",
        )
        .unwrap();

        assert_eq!(
            vec![
                "2019-04-02 09:12:00".to_owned(),
                "2019-04-09 09:12:00".to_owned(),
                "2019-04-16 09:12:00".to_owned(),
                "2019-04-23 09:12:00".to_owned(),
                "2019-04-30 09:12:00".to_owned(),
                "2019-05-07 09:12:00".to_owned(),
                "2019-05-14 09:12:00".to_owned(),
                "2019-05-21 09:12:00".to_owned(),
                "2019-05-28 09:12:00".to_owned(),
                "2019-06-04 09:12:00".to_owned(),
                "2019-06-11 09:12:00".to_owned(),
                "2019-06-18 09:12:00".to_owned(),
                "2019-06-25 09:12:00".to_owned(),
                "2019-07-02 09:12:00".to_owned(),
                "2019-07-09 09:12:00".to_owned(),
                "2019-07-16 09:12:00".to_owned(),
                "2019-07-23 09:12:00".to_owned(),
                "2019-07-30 09:12:00".to_owned(),
                "2019-08-06 09:12:00".to_owned(),
                "2019-08-13 09:12:00".to_owned(),
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_hourly_rules_work_3() {
        let rrule_result = convert_to_rrule("FREQ=HOURLY;INTERVAL=3;COUNT=20;BYDAY=TH;DTSTART=20190327T030000;TZID=Australia/Sydney").unwrap();

        assert_eq!(
            vec![
                "2019-03-28T02:00:00+11:00".to_owned(),
                "2019-03-28T05:00:00+11:00".to_owned(),
                "2019-03-28T08:00:00+11:00".to_owned(),
                "2019-03-28T11:00:00+11:00".to_owned(),
                "2019-03-28T14:00:00+11:00".to_owned(),
                "2019-03-28T17:00:00+11:00".to_owned(),
                "2019-03-28T20:00:00+11:00".to_owned(),
                "2019-03-28T23:00:00+11:00".to_owned(),
                "2019-04-04T02:00:00+11:00".to_owned(),
                "2019-04-04T05:00:00+11:00".to_owned(),
                "2019-04-04T08:00:00+11:00".to_owned(),
                "2019-04-04T11:00:00+11:00".to_owned(),
                "2019-04-04T14:00:00+11:00".to_owned(),
                "2019-04-04T17:00:00+11:00".to_owned(),
                "2019-04-04T20:00:00+11:00".to_owned(),
                "2019-04-04T23:00:00+11:00".to_owned(),
                "2019-04-11T02:00:00+11:00".to_owned(),
                "2019-04-11T05:00:00+10:00".to_owned(),
                "2019-04-11T08:00:00+10:00".to_owned(),
                "2019-04-11T11:00:00+10:00".to_owned(),
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        );
    }

    #[test]
    fn test_minutely_rules_work_1() {
        let rrule_result =
            convert_to_rrule("FREQ=MINUTELY;INTERVAL=3;COUNT=20;DTSTART=20190327T030000").unwrap();

        assert_eq!(
            vec![
                "2019-03-27 03:03:00".to_owned(),
                "2019-03-27 03:06:00".to_owned(),
                "2019-03-27 03:09:00".to_owned(),
                "2019-03-27 03:12:00".to_owned(),
                "2019-03-27 03:15:00".to_owned(),
                "2019-03-27 03:18:00".to_owned(),
                "2019-03-27 03:21:00".to_owned(),
                "2019-03-27 03:24:00".to_owned(),
                "2019-03-27 03:27:00".to_owned(),
                "2019-03-27 03:30:00".to_owned(),
                "2019-03-27 03:33:00".to_owned(),
                "2019-03-27 03:36:00".to_owned(),
                "2019-03-27 03:39:00".to_owned(),
                "2019-03-27 03:42:00".to_owned(),
                "2019-03-27 03:45:00".to_owned(),
                "2019-03-27 03:48:00".to_owned(),
                "2019-03-27 03:51:00".to_owned(),
                "2019-03-27 03:54:00".to_owned(),
                "2019-03-27 03:57:00".to_owned(),
                "2019-03-27 04:00:00".to_owned()
            ],
            rrule_result
                .get_all_iter_dates("", "")
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_weekly_rrules_1() {
        let mut rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=1;COUNT=5;BYDAY=TH;BYHOUR=14;BYMINUTE=0;BYSECOND=0;TZID=Australia/Sydney;DTSTART=20190415T160000")
                .unwrap();
        assert_eq!(
            vec![
                "2019-04-18T14:00:00+10:00",
                "2019-04-25T14:00:00+10:00",
                "2019-05-02T14:00:00+10:00",
                "2019-05-09T14:00:00+10:00",
                "2019-05-16T14:00:00+10:00",
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_weekly_rrules_2() {
        let mut rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=1;COUNT=3;BYDAY=MO;BYHOUR=22;BYMINUTE=0;BYSECOND=30;DTSTART=20190415T031500")
                .unwrap();
        assert_eq!(
            vec![
                "2019-04-15T22:00:30+00:00",
                "2019-04-22T22:00:30+00:00",
                "2019-04-29T22:00:30+00:00",
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_weekly_rrules_3() {
        let rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=1;COUNT=4;BYDAY=WE;BYHOUR=14;BYMINUTE=55;BYSECOND=0;TZID=Australia/Sydney;DTSTART=20190411T000000")
                .unwrap();
        assert_eq!(
            vec![
                "2019-04-17T14:55:00+10:00",
                "2019-04-24T14:55:00+10:00",
                "2019-05-01T14:55:00+10:00",
                "2019-05-08T14:55:00+10:00",
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_weekly_rrules_4() {
        let rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=1;COUNT=3;BYDAY=TU;BYHOUR=23;BYMINUTE=54;BYSECOND=0;TZID=Australia/Melbourne;DTSTART=20190410T034500")
                .unwrap();
        assert_eq!(
            vec![
                "2019-04-16T23:54:00+10:00",
                "2019-04-23T23:54:00+10:00",
                "2019-04-30T23:54:00+10:00",
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_fortnightly_rrules_1() {
        // test case group 1 - implicit by day from dtStart
        let rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=0;BYSECOND=48;TZID=Australia/West;DTSTART=20190101T031500").unwrap();
        let iter_dates = rrule_result.get_all_iter_dates("", "");
        for date in iter_dates.iter() {
            assert_eq!(Weekday::Tue, date.weekday());
            assert_eq!(00, date.hour());
            assert_eq!(15, date.minute());
            assert_eq!(48, date.second());
        }
    }

    #[test]
    fn test_fortnightly_rules_2() {
        let rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=17;BYMINUTE=0;TZID=Australia/Sydney;DTSTART=20181122T000003").unwrap();
        let iter_dates = rrule_result.get_all_iter_dates("", "");
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Thu, date.weekday());
            assert_eq!(17, date.hour());
            assert_eq!(00, date.minute());
            assert_eq!(03, date.second());
        }
    }

    #[test]
    fn test_fortnightly_rules_3() {
        let rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=17;BYMINUTE=0;TZID=Australia/Sydney;DTSTART=20181122T000003").unwrap();
        let iter_dates = rrule_result.get_all_iter_dates("", "");
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Thu, date.weekday());
            assert_eq!(17, date.hour());
            assert_eq!(00, date.minute());
            assert_eq!(03, date.second());
        }
    }

    #[test]
    fn test_fortnightly_rrules_3() {
        let rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;COUNT=5;BYHOUR=10;BYMINUTE=30;BYSECOND=0;TZID=Australia/Sydney;DTSTART=20190411T000000")
                .unwrap();
        assert_eq!(
            vec![
                "2019-04-11T10:30:00+10:00",
                "2019-04-25T10:30:00+10:00",
                "2019-05-09T10:30:00+10:00",
                "2019-05-23T10:30:00+10:00",
                "2019-06-06T10:30:00+10:00",
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_1() {
        // test we get the right next date
        let rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28;DTSTART=20190315T011213;TZID=UTC").unwrap();
        assert_eq!(
            vec!["2019-03-28T09:01:13+00:00".to_owned()],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_2() {
        // test we get the right next date
        let rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=2;BYMONTHDAY=28;DTSTART=20190402T011213;TZID=Australia/Melbourne").unwrap();
        assert_eq!(
            vec![
                "2019-04-28T12:12:13+10:00".to_owned(),
                "2019-05-28T12:12:13+10:00".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_3() {
        // test we get the right next date
        let rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=12;BYMONTH=6;DTSTART=20190402T011213;TZID=Australia/Melbourne").unwrap();
        assert_eq!(
            vec![
                "2019-06-02T12:12:13+10:00".to_owned(),
                "2020-06-02T12:12:13+10:00".to_owned(),
                "2021-06-02T12:12:13+10:00".to_owned(),
                "2022-06-02T12:12:13+10:00".to_owned(),
                "2023-06-02T12:12:13+10:00".to_owned(),
                "2024-06-02T12:12:13+10:00".to_owned(),
                "2025-06-02T12:12:13+10:00".to_owned(),
                "2026-06-02T12:12:13+10:00".to_owned(),
                "2027-06-02T12:12:13+10:00".to_owned(),
                "2028-06-02T12:12:13+10:00".to_owned(),
                "2029-06-02T12:12:13+10:00".to_owned(),
                "2030-06-02T12:12:13+10:00".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_4() {
        // test we get the right next date
        let rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=12;BYMONTH=11;BYMONTHDAY=12;DTSTART=20190402T011213;TZID=Australia/Sydney").unwrap();
        assert_eq!(
            vec![
                "2019-11-12T12:12:13+11:00".to_owned(),
                "2020-11-12T12:12:13+11:00".to_owned(),
                "2021-11-12T12:12:13+11:00".to_owned(),
                "2022-11-12T12:12:13+11:00".to_owned(),
                "2023-11-12T12:12:13+11:00".to_owned(),
                "2024-11-12T12:12:13+11:00".to_owned(),
                "2025-11-12T12:12:13+11:00".to_owned(),
                "2026-11-12T12:12:13+11:00".to_owned(),
                "2027-11-12T12:12:13+11:00".to_owned(),
                "2028-11-12T12:12:13+11:00".to_owned(),
                "2029-11-12T12:12:13+11:00".to_owned(),
                "2030-11-12T12:12:13+11:00".to_owned()
            ],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_5() {
        // test we get the right next date
        let rrule_result =
            convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=1;BYMONTHDAY=12").unwrap();
        let test_start_date = Utc
            .ymd(2019, 04, 13)
            .and_hms(01, 12, 13)
            .with_timezone(&UTC);
        let expected_next_date = Utc
            .ymd(2019, 05, 12)
            .and_hms(01, 12, 13)
            .with_timezone(&UTC);
        assert_eq!(
            expected_next_date,
            rrule_result
                .get_next_date(test_start_date)
                .with_timezone(&UTC)
        )
    }

    #[test]
    fn test_monthly_rrule_by_month_day_edge_cases() {
        let rrule_result =
            convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=1;BYMONTHDAY=12;BYHOUR=17").unwrap();
        let cases = vec![
            (
                Utc.ymd(2019, 04, 12)
                    .and_hms(01, 12, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2019, 04, 12)
                    .and_hms(17, 12, 13)
                    .with_timezone(&UTC),
            ),
            (
                Utc.ymd(2019, 04, 12)
                    .and_hms(18, 12, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2019, 05, 12)
                    .and_hms(17, 12, 13)
                    .with_timezone(&UTC),
            ),
            (
                Utc.ymd(2019, 04, 12)
                    .and_hms(17, 12, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2019, 05, 12)
                    .and_hms(17, 12, 13)
                    .with_timezone(&UTC),
            ),
        ];

        for case in cases {
            assert_eq!(
                case.1,
                rrule_result.get_next_date(case.0).with_timezone(&UTC)
            )
        }
    }

    #[test]
    fn test_monthly_rrule_by_month_edge_cases() {
        // test we get the right next date
        let rrule_result =
            convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=1;BYMONTH=4;BYMONTHDAY=13;BYHOUR=17")
                .unwrap();
        let cases = vec![
            (
                Utc.ymd(2019, 04, 12)
                    .and_hms(17, 13, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2019, 04, 13)
                    .and_hms(17, 13, 13)
                    .with_timezone(&UTC),
            ),
            (
                Utc.ymd(2019, 04, 13)
                    .and_hms(17, 13, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2020, 04, 13)
                    .and_hms(17, 13, 13)
                    .with_timezone(&UTC),
            ),
            (
                Utc.ymd(2019, 04, 13)
                    .and_hms(18, 13, 13)
                    .with_timezone(&UTC),
                Utc.ymd(2020, 04, 13)
                    .and_hms(17, 13, 13)
                    .with_timezone(&UTC),
            ),
        ];

        for case in cases {
            assert_eq!(
                case.1,
                rrule_result.get_next_date(case.0).with_timezone(&UTC)
            )
        }
    }

    #[test]
    fn we_support_yearly_rules_properly() {
        // test we get the right next date
        let rrule_result = convert_to_rrule("FREQ=YEARLY;COUNT=2;INTERVAL=1").unwrap();
        let test_start_date = Utc
            .ymd(2019, 03, 15)
            .and_hms(01, 12, 13)
            .with_timezone(&UTC);
        assert_eq!(
            test_start_date.with_year(2020).unwrap(),
            rrule_result
                .get_next_date(test_start_date)
                .with_timezone(&UTC)
        )
    }

    #[test]
    fn we_can_deserialize_rrule_json_succesfully_1() {
        // test we get the right next date
        let rrule_expected = convert_to_rrule("FREQ=YEARLY;COUNT=2;INTERVAL=1").unwrap();
        let rrule_1 = rrule_expected.to_json();
        let rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref()).unwrap();
        assert_eq!(rrule_actual_1, rrule_expected);
    }

    #[test]
    fn we_can_deserialize_rrule_json_succesfully_2() {
        // test we get the right next date
        let rrule_expected = convert_to_rrule(
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        )
        .unwrap();
        let rrule_1 = rrule_expected.to_json();
        let rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref()).unwrap();
        assert_eq!(rrule_actual_1, rrule_expected)
    }

    #[test]
    fn we_can_scope_returned_results_8601_from_today() {
        let rrule_result =
            convert_to_rrule("FREQ=WEEKLY;INTERVAL=1;COUNT=3;BYDAY=TU;BYHOUR=23;BYMINUTE=54;BYSECOND=0;TZID=Australia/Melbourne;DTSTART=20180110T034500")
                .unwrap();

        assert_eq!(
            3,
            rrule_result
                .get_all_iter_dates_from_today_iso8601("", "")
                .len()
        )
    }
}
