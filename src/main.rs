#[macro_use]
extern crate pest_derive;

use serde::Serialize;
use serde_json::json;
use std::env;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rrule.pest"]
struct RRuleParser;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RRule<'a> {
    #[serde(skip_serializing_if = "String::is_empty")]
    frequency: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    count: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    interval: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_hour: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_minute: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_second: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_day: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_month_day: Vec<&'a str>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_year_day: Vec<&'a str>,
}

impl<'a> RRule<'a> {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

use pest::iterators::Pair;
use std::collections::HashMap;

/// converts and rrule string to a rrule struct
fn convert_to_rrule<'a>(rrule_result: &mut RRule<'a>, rrule_string: &'a str) {
    let parse_result = RRuleParser::parse(Rule::expr, rrule_string)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for line in parse_result.into_inner() {
        match line.as_rule() {
            Rule::freq_exprs => {
                rrule_result.frequency = line.into_inner().next().unwrap().as_str().to_string();
            }

            Rule::interval_expr => {
                rrule_result.interval = line.into_inner().next().unwrap().as_str().to_string();
            }

            Rule::count_expr => {
                rrule_result.count = line.into_inner().next().unwrap().as_str().to_string();
            }

            Rule::byhour_expr => {
                rrule_result.by_hour = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }

            Rule::byminute_expr => {
                rrule_result.by_minute = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }

            Rule::bysecond_expr => {
                rrule_result.by_second = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }

            Rule::byday_expr => {
                rrule_result.by_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }

            Rule::bymonthday_expr => {
                rrule_result.by_month_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }

            Rule::byyearday_expr => {
                rrule_result.by_year_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
            }
            _ => {}
        }
    }
}

// ToDo : Add validation for checking that the RRULE string was properly extracted from the parser
// by counting ';' in the original rrule string and ':' in the parsed json
fn main() {
    let args: Vec<String> = env::args().collect();

    let s = "FREQ=DAILY".to_owned();
    let mut rrule_result = RRule {
        frequency: String::from(""),
        count: String::from(""),
        interval: String::from(""),
        by_hour: Vec::new(),
        by_minute: Vec::new(),
        by_second: Vec::new(),
        by_day: Vec::new(),
        by_month_day: Vec::new(),
        by_year_day: Vec::new(),
    };

    convert_to_rrule(&mut rrule_result, &s);
    println!("Json is {}", rrule_result.to_json());
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_rrule, RRule};
    use serde_json::json;

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
        ];

        for i in &rrule_test_cases {
            let mut rrule_result = RRule {
                frequency: String::from(""),
                count: String::from(""),
                interval: String::from(""),
                by_hour: Vec::new(),
                by_minute: Vec::new(),
                by_second: Vec::new(),
                by_day: Vec::new(),
                by_month_day: Vec::new(),
                by_year_day: Vec::new(),
            };

            convert_to_rrule(&mut rrule_result, i.rrule_string);

            assert_eq!(i.expected_flat_json, rrule_result.to_json())
        }
    }
}
