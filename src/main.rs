#[macro_use]
extern crate pest_derive;

use chrono::prelude::*;
use pest::Parser;
use serde::Serialize;
use serde_json::json;
use std::env;

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
    #[serde(skip_serializing_if = "String::is_empty")]
    wkst: String,
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

    // show me the money
    // parent function that can get a list of all future iterations based on count
    fn get_next_iter_dates(&self) -> Vec<DateTime<Utc>> {
        // ToDo: need to add dtStart to parser and don't assume today as the starting date
        let dt_start = Utc::now();
        let mut count: i8 = 52; // default count of iterations to build

        // assign default weekstart and reassign if present
        let mut wkst = "MO";

        if !self.wkst.is_empty() {
            wkst = &self.wkst
        }

        if !self.count.is_empty() {
            count = self.count.parse().unwrap()
        }

        let mut dates_to_return: Vec<DateTime<Utc>> = Vec::new();
        let mut new_start_date = dt_start;
        for i in 0..count {
            new_start_date = self.get_next_date(new_start_date);
            dates_to_return.push(new_start_date);
        }

        dates_to_return
    }

    // standalone function that gets iterations from a single start date
    fn get_next_date(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut return_date = start_date;
        // handle yearly
        if self.frequency.eq("YEARLY") {
            return_date = self.handle_yearly(start_date)
        // handle monthly
        } else if self.frequency == "MONTHLY" {
            return_date = self.handle_monthly(start_date)
        } else {
            println!("given RRule format has not been implemented yet")
        }
        return_date
    }

    // set the lower interval time for start date
    fn set_initial_time_intervals(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut start_date_with_intervals = start_date;

        if self.frequency.ne("SECONDLY") {
            let mut second: u32 = start_date.second();
            if !self.by_second.is_empty() {
                second = self.by_second.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals.with_second(second);
        }

        if self.frequency.ne("SECONDLY") && self.frequency.ne("MONTHLY") {
            let mut minute: u32 = start_date.minute();
            if !self.by_minute.is_empty() {
                minute = self.by_minute.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals.with_minute(minute);
        }

        if self.frequency.ne("SECONDLY")
            && self.frequency.ne("MONTHLY")
            && self.frequency.ne("HOURLY")
        {
            let mut hour: u32 = start_date.hour();
            if !self.by_hour.is_empty() {
                hour = self.by_hour.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals.with_hour(hour);
        }

        start_date_with_intervals
    }

    // handles the calculation of next date based on a monthly rule
    fn handle_monthly(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut next_date = start_date;
        let panic_value: u32 = 50;
        // use 50 as panic day as it resides outside the bound of permissible parse range for byMonthDay
        // expression
        let month_day: u32 = self.by_month_day.first().unwrap_or(&"50").parse().unwrap();
        if month_day.eq(&panic_value) {
            // go crazy, we don't like this since if you're asking me to process monthly
            // I will need a bymonth day present and there is no way I can recover from this
            panic!("Need a bymonth rrule part when evaluation rules with monthly freq");
        } else {
            let start_date_day = start_date.day();
            if start_date_day < month_day {
                next_date = next_date.with_day(month_day).unwrap()
            } else if start_date_day > month_day {
                if start_date.month().lt(&(12 as u32)) {
                    let mut month_added = next_date.with_month(start_date.month() + 1).unwrap();
                    next_date = month_added.with_day(month_day).unwrap();
                } else {
                    let mut year_added = next_date.with_year(start_date.year() + 1).unwrap();
                    let mut month_added = year_added.with_month(1).unwrap();
                    next_date = month_added.with_day(month_day).unwrap();
                }
            } else if start_date_day == month_day {
                let start_date_with_intervals = self.set_initial_time_intervals(start_date);

                // even if its the same day, if we've shot past the time, we will need to schedule
                // for next month
                if start_date_with_intervals.ge(&start_date) {
                    if start_date.month().lt(&(12 as u32)) {
                        let mut month_added = next_date.with_month(start_date.month() + 1).unwrap();
                        next_date = month_added.with_day(month_day).unwrap();
                    } else {
                        let mut year_added = next_date.with_year(start_date.year() + 1).unwrap();
                        let mut month_added = year_added.with_month(1).unwrap();
                        next_date = month_added.with_day(month_day).unwrap();
                    }
                }
            }
        }
        next_date
    }

    // currently only supports rrules of type: REQ=YEARLY;COUNT=x;INTERVAL=x
    fn handle_yearly(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut interval: u32 = self.interval.parse().unwrap();
        let max_year = 2099;
        let mut next_date = start_date;
        let mut next_year = start_date.year() + 1;
        for i in 0..interval {
            if next_date.year().lt(&(max_year as i32)) {
                next_date = next_date.with_year(next_year).unwrap()
            }
            next_year = next_year + 1;
        }
        next_date
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
            Rule::freq_expr => {
                rrule_result.frequency = line.into_inner().next().unwrap().as_str().to_string();
            }

            Rule::interval_expr => {
                rrule_result.interval = line.into_inner().next().unwrap().as_str().to_string();
            }

            Rule::wkst_expr => {
                rrule_result.wkst = line.into_inner().next().unwrap().as_str().to_string();
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
    let s = "FREQ=YEARLY;COUNT=2;INTERVAL=1".to_owned();
    let mut rrule_result = RRule {
        frequency: String::from(""),
        count: String::from(""),
        interval: String::from(""),
        wkst: String::from(""),
        by_hour: Vec::new(),
        by_minute: Vec::new(),
        by_second: Vec::new(),
        by_day: Vec::new(),
        by_month_day: Vec::new(),
        by_year_day: Vec::new(),
    };

    convert_to_rrule(&mut rrule_result, &s);
    println!("next date is {}", rrule_result.get_next_date(Utc::now()));
    println!("next dates are {:?}", rrule_result.get_next_iter_dates());
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_rrule, RRule};
    use chrono::offset::TimeZone;
    use chrono::{Datelike, Utc};
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
                wkst: String::from(""),
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

    #[test]
    fn test_we_use_the_count_properly() {
        let mut rrule_result = RRule {
            frequency: String::from(""),
            count: String::from(""),
            interval: String::from(""),
            wkst: String::from(""),
            by_hour: Vec::new(),
            by_minute: Vec::new(),
            by_second: Vec::new(),
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };

        // test we get the right next date
        convert_to_rrule(
            &mut rrule_result,
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        );
        assert_eq!(27, rrule_result.get_next_iter_dates().len())
    }

    #[test]
    fn test_monthly_rrule() {
        let mut rrule_result = RRule {
            frequency: String::from(""),
            count: String::from(""),
            interval: String::from(""),
            wkst: String::from(""),
            by_hour: Vec::new(),
            by_minute: Vec::new(),
            by_second: Vec::new(),
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };
        // test we get the right next date
        convert_to_rrule(
            &mut rrule_result,
            "FREQ=MONTHLY;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        );
        let mut test_start_date = Utc.ymd(2019, 03, 15).and_hms(01, 12, 13);
        assert_eq!(
            test_start_date.with_day(28).unwrap(),
            rrule_result.get_next_date(test_start_date)
        )
    }

    #[test]
    fn we_support_yearly_rules_properly() {
        let mut rrule_result = RRule {
            frequency: String::from(""),
            count: String::from(""),
            interval: String::from(""),
            wkst: String::from(""),
            by_hour: Vec::new(),
            by_minute: Vec::new(),
            by_second: Vec::new(),
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };

        // test we get the right next date
        convert_to_rrule(&mut rrule_result, "FREQ=YEARLY;COUNT=2;INTERVAL=1");
        let mut test_start_date = Utc.ymd(2019, 03, 15).and_hms(01, 12, 13);
        assert_eq!(
            test_start_date.with_year(2020).unwrap(),
            rrule_result.get_next_date(test_start_date)
        )
    }
}
