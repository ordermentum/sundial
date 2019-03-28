#[macro_use]
extern crate pest_derive;

use chrono::prelude::*;
use chrono::{Duration, TimeZone};
use chrono_tz::Tz;
use chrono_tz::UTC;
use pest::Parser;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use serde_json::Result;
use std::env;
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "rrule.pest"]
struct RRuleParser;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
struct RRule<'a> {
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    tzid: String,
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    dtstart: String,
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    frequency: String,
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    count: String,
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    interval: String,
    #[serde(default = "default_rrule_string_field")]
    #[serde(skip_serializing_if = "String::is_empty")]
    wkst: String,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_month: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_hour: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_minute: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_second: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_day: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_month_day: Vec<&'a str>,
    #[serde(default = "default_rrule_vec_field")]
    #[serde(borrow)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    by_year_day: Vec<&'a str>,
}

fn default_rrule_string_field() -> String {
    "".to_string()
}

fn default_rrule_vec_field<'a>() -> Vec<&'a str> {
    Vec::new()
}

impl<'a> RRule<'a> {
    /// Generates a new empty RRule instance
    ///
    /// Example:
    /// ```
    /// let rrule = RRule::new();
    /// ```
    #[inline]
    pub fn new<'b>() -> RRule<'b> {
        return RRule {
            tzid: String::from(""),
            dtstart: String::from(""),
            frequency: String::from(""),
            count: String::from(""),
            interval: String::from(""),
            wkst: String::from(""),
            by_month: Vec::new(),
            by_hour: Vec::new(),
            by_minute: Vec::new(),
            by_second: Vec::new(),
            by_day: Vec::new(),
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };
    }

    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    // show me the money
    // parent function that can get a list of all future iterations based on count
    fn get_all_iter_dates(&self) -> Vec<DateTime<Utc>> {
        // ToDo: Need to respect timezones and not assume UTC
        let mut start_date = if self.dtstart.is_empty() {
            Utc::now()
        } else {
            Utc.datetime_from_str(&self.dtstart, "%Y-%m-%d %H:%M:%S")
                .unwrap() as DateTime<Utc>
        };

        let mut count: i8 = 52; // default count of iterations to build

        // assign default weekstart and reassign if present
        let mut wkst = "MO";

        if !self.wkst.is_empty() {
            wkst = &self.wkst
        }

        if !self.count.is_empty() {
            count = self.count.parse().unwrap()
        }

        let mut next_dates_list: Vec<DateTime<Utc>> = Vec::new();
        let mut next_date = start_date;
        for i in 0..count {
            next_date = self.get_next_date(next_date);
            next_dates_list.push(next_date);
        }
        next_dates_list
    }

    fn get_next_iter_dates(&self) -> Vec<DateTime<Utc>> {
        lens_iter_dates(self.get_all_iter_dates(), Utc::now())
    }

    // standalone function that gets iterations from a single start date
    fn get_next_date(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut return_date = start_date;
        // handle yearly
        if self.frequency.eq("YEARLY") {
            return_date = self.handle_yearly(start_date)
        } else if self.frequency == "MONTHLY" {
            return_date = self.handle_monthly(start_date)
        } else if self.frequency == "WEEKLY" {
            return_date = self.handle_weekly(start_date)
        } else if self.frequency == "DAILY" {
            return_date = self.handle_daily(start_date)
        } else {
            println!("Given RRule format has not been implemented yet")
        }
        return_date
    }

    // set the lower interval time for start date
    fn with_initial_time_intervals(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut start_date_with_intervals = start_date;

        if self.frequency.ne("SECONDLY") {
            let mut second: u32 = start_date.second();
            if !self.by_second.is_empty() {
                second = self.by_second.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals = start_date_with_intervals.with_second(second).unwrap();
        }

        if self.frequency.ne("SECONDLY") && self.frequency.ne("MONTHLY") {
            let mut minute: u32 = start_date.minute();
            if !self.by_minute.is_empty() {
                minute = self.by_minute.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals = start_date_with_intervals.with_minute(minute).unwrap();
        }

        if self.frequency.ne("SECONDLY")
            && self.frequency.ne("MONTHLY")
            && self.frequency.ne("HOURLY")
        {
            let mut hour: u32 = start_date.hour();
            if !self.by_hour.is_empty() {
                hour = self.by_hour.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals = start_date_with_intervals.with_hour(hour).unwrap();
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
                let start_date_with_intervals = self.with_initial_time_intervals(start_date);

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

    /// Handles both weekly and special variants of weekly such as [FREQ=WEEKLY;INTERVAL=2;]
    /// which can colloquially evaluate to fortnightly.
    /// Pseudo Code (Todo: Delete Later)
    /// -- for only one byWeekDay
    /// adjust start_date day to match the byweekday
    /// for i in 0..interval
    ///     add 7 days to the current_date
    ///     if not(check if it falls on the right byWkDay) -> adjust to the right wkday
    fn handle_weekly(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut start_date_with_intervals = self.with_initial_time_intervals(start_date);
        // adjust start_date if it does not start on the start
        let panic_value = "PA";
        // use 50 as panic day as it resides outside the bound of permissible parse range for byMonthDay
        // expression
        let by_day = self
            .by_day
            .first()
            .unwrap_or(&chrono_weekday_to_rrule_byday(start_date.weekday()))
            .to_owned();

        if by_day.eq("PA") {
            // go crazy we don't like this
            panic!("Need a byDay rrule part when evaluation rules with weekly freq")
        } else {
            // now adjust the date to match the start day
            let in_future = start_date_with_intervals.gt(&start_date);
            let days_to_adjust =
                self.calculate_weekday_distance(by_day, start_date.weekday(), in_future);
            start_date_with_intervals = start_date_with_intervals + Duration::days(days_to_adjust);

            let mut interval: u32 = self.interval.parse().unwrap();
            let mut next_date = start_date_with_intervals;
            for i in 0..interval {
                next_date = next_date + Duration::days(7);
            }
            let final_days_to_adjust =
                self.calculate_weekday_distance(by_day, next_date.weekday(), false);
            // do a final adjustment in case we are going over monthly boundaries
            // and the calculated date day does not coincide with the one provided
            // by the client
            next_date = next_date + Duration::days(final_days_to_adjust);
            next_date
        }
    }

    fn handle_daily(&self, start_date: DateTime<Utc>) -> DateTime<Utc> {
        let mut start_date_with_intervals = self.with_initial_time_intervals(start_date);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        let mut interval: u32 = self.interval.parse().unwrap();
        let mut next_date = start_date_with_intervals;

        if by_month.is_empty() {
            if by_day.is_empty() {
                println!("inside empty byday block");
                for i in 0..interval {
                    next_date = next_date + Duration::days(1);
                }
            } else {
                loop {
                    for i in 0..interval {
                        next_date = next_date + Duration::days(1);
                    }
                    if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                        break;
                    }
                }
            }
        } else {
            if by_day.is_empty() {
                println!("inside empty byday block");
                for i in 0..interval {
                    next_date = next_date + Duration::days(1);
                }
            } else {
                loop {
                    for i in 0..interval {
                        next_date = next_date + Duration::days(1);
                    }
                    if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day)
                        && next_date.month().eq(&(by_month.parse::<u32>().unwrap()))
                    {
                        break;
                    }
                }
            }
        }
        next_date
    }

    /// Calculates the weekdays to add based on the given byweekday and current weekday.
    /// Use the `in_future_from_current_day` property to determine whether we should use the
    /// current day or the day in future.
    /// A simple example is, lets say the byWkDay is `TU` and current weekday is also `Tuesday`
    /// however, the `DTSTART` of the rule is 20180326T160003 but the complete dtStart for the
    /// the rule is 20180326T160000 based on lets say the bySecond property being 0, then the
    /// date is in the future and we should use next tuesday as the start date instead of the
    /// current tuesday
    fn calculate_weekday_distance(
        &self,
        bywk_day: &str,
        current_weekday: Weekday,
        in_future_from_current_day: bool,
    ) -> i64 {
        let number_from_mon = current_weekday.number_from_monday();
        let mut adjustment: i64 = 0;
        match bywk_day {
            "MO" => {
                match number_from_mon {
                    1 => {
                        // monday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    2 => {
                        // tuesday
                        adjustment = 6;
                    }
                    3 => {
                        // wednesday
                        adjustment = 5;
                    }
                    4 => {
                        // thursday
                        adjustment = 4;
                    }
                    5 => {
                        // friday
                        adjustment = 3;
                    }
                    6 => {
                        // saturday
                        adjustment = 2;
                    }
                    7 => {
                        // sunday
                        adjustment = 1;
                    }
                    _ => {}
                }
            }
            "TU" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 1;
                    }
                    2 => {
                        // tuesday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    3 => {
                        // wednesday
                        adjustment = 6;
                    }
                    4 => {
                        // thursday
                        adjustment = 5;
                    }
                    5 => {
                        // friday
                        adjustment = 4;
                    }
                    6 => {
                        // saturday
                        adjustment = 3;
                    }
                    7 => {
                        // sunday
                        adjustment = 2;
                    }
                    _ => {}
                }
            }
            "WE" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 2;
                    }
                    2 => {
                        // tuesday
                        adjustment = 1;
                    }
                    3 => {
                        // wednesday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    4 => {
                        // thursday
                        adjustment = 6;
                    }
                    5 => {
                        // friday
                        adjustment = 5;
                    }
                    6 => {
                        // saturday
                        adjustment = 4;
                    }
                    7 => {
                        // sunday
                        adjustment = 3;
                    }
                    _ => {}
                }
            }
            "TH" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 3;
                    }
                    2 => {
                        // tuesday
                        adjustment = 2;
                    }
                    3 => {
                        // wednesday
                        adjustment = 1;
                    }
                    4 => {
                        // thursday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    5 => {
                        // friday
                        adjustment = 6;
                    }
                    6 => {
                        // saturday
                        adjustment = 5;
                    }
                    7 => {
                        // sunday
                        adjustment = 4;
                    }
                    _ => {}
                }
            }
            "FR" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 4;
                    }
                    2 => {
                        // tuesday
                        adjustment = 3;
                    }
                    3 => {
                        // wednesday
                        adjustment = 2;
                    }
                    4 => {
                        // thursday
                        adjustment = 1;
                    }
                    5 => {
                        // friday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    6 => {
                        // saturday
                        adjustment = 6;
                    }
                    7 => {
                        // sunday
                        adjustment = 5;
                    }
                    _ => {}
                }
            }
            "SA" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 5;
                    }
                    2 => {
                        // tuesday
                        adjustment = 4;
                    }
                    3 => {
                        // wednesday
                        adjustment = 3;
                    }
                    4 => {
                        // thursday
                        adjustment = 2;
                    }
                    5 => {
                        // friday
                        adjustment = 1;
                    }
                    6 => {
                        // saturday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    7 => {
                        // sunday
                        adjustment = 6;
                    }
                    _ => {}
                }
            }
            "SU" => {
                match number_from_mon {
                    1 => {
                        // monday
                        adjustment = 6;
                    }
                    2 => {
                        // tuesday
                        adjustment = 5;
                    }
                    3 => {
                        // wednesday
                        adjustment = 4;
                    }
                    4 => {
                        // thursday
                        adjustment = 3;
                    }
                    5 => {
                        // friday
                        adjustment = 2;
                    }
                    6 => {
                        // saturday
                        adjustment = 1;
                    }
                    7 => {
                        // sunday
                        if in_future_from_current_day {
                            adjustment = 7;
                        } else {
                            adjustment = 0;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        adjustment
    }
}

fn chrono_weekday_to_rrule_byday(weekday: Weekday) -> &'static str {
    let mut weekday_string = "";
    match weekday {
        Weekday::Mon => {
            weekday_string = "MO";
        }
        Weekday::Tue => {
            weekday_string = "TU";
        }
        Weekday::Wed => {
            weekday_string = "WE";
        }
        Weekday::Thu => {
            weekday_string = "TH";
        }
        Weekday::Fri => {
            weekday_string = "FR";
        }
        Weekday::Sat => {
            weekday_string = "SA";
        }
        Weekday::Sun => {
            weekday_string = "SU";
        }
    }
    weekday_string
}
/// Given a `dates_list` of future iteration dates and a `lens_from_date` to look
/// forward from, this function
/// selects the dates that are strictly in the future and returns a modified list
/// with past dates removed.
fn lens_iter_dates(
    dates_list: Vec<DateTime<Utc>>,
    lens_from_date: DateTime<Utc>,
) -> Vec<DateTime<Utc>> {
    let mut lensed_dates_list: Vec<DateTime<Utc>> = Vec::new();
    for date in dates_list.iter() {
        if date.gt(&lens_from_date) {
            // only add dates in the future
            lensed_dates_list.push(date.to_owned())
        }
    }
    lensed_dates_list
}

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

use pest::iterators::Pair;
use std::collections::HashMap;

/// Converts and rrule string to a rrule struct
fn convert_to_rrule<'a>(rrule_result: &mut RRule<'a>, rrule_string: &'a str) {
    let parse_result = RRuleParser::parse(Rule::expr, rrule_string)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for line in parse_result.into_inner() {
        match line.as_rule() {
            Rule::tz_expr => {
                // parse timezone
                let tz_unparsed: String = line.into_inner().next().unwrap().as_str().to_string();
                // ToDo: add a check using chrono time zone to check if the timezone is parseable
                rrule_result.tzid = tz_unparsed;
            }

            Rule::dtstart_expr_with_tz => {
                // For when dtstart had timezone provided
                // Todo: Heaps of failure points here, add error handling; actually the whole filed needs it
                if rrule_result.dtstart.is_empty() {
                    // only one instance of dtStart is allowed and according to
                    // the spec any errors should be silently dropped when parsing
                    let non_validated_dtstart: String =
                        line.into_inner().next().unwrap().as_str().to_string();
                    let tz_split: Vec<&str> = non_validated_dtstart.split(":").collect();
                    if tz_split.len() > 1 {
                        // we have time zone
                        let tz = tz_split[0];
                        let timezone = chrono_tz::Tz::from_str(tz).unwrap();
                        let naive_date =
                            NaiveDateTime::parse_from_str(tz_split[1], "%Y%m%dT%H%M%S").unwrap();
                        rrule_result.dtstart = timezone
                            .from_local_datetime(&naive_date)
                            .unwrap()
                            .to_string();
                    } else {
                        panic!("Invalid DTSTART;TZID string {}", non_validated_dtstart)
                    }
                }
            }

            Rule::dtstart_expr_without_tz => {
                // assume UTC if not provided
                if rrule_result.dtstart.is_empty() {
                    let mut non_validated_dtstart: String =
                        line.into_inner().next().unwrap().as_str().to_string();
                    if non_validated_dtstart.contains("Z") {
                        let naive_date =
                            NaiveDateTime::parse_from_str(&non_validated_dtstart, "%Y%m%dT%H%M%SZ")
                                .unwrap();
                        rrule_result.dtstart = naive_date.to_string();
                    } else {
                        // no tzId specified, use UTC
                        let naive_date =
                            NaiveDateTime::parse_from_str(&non_validated_dtstart, "%Y%m%dT%H%M%S")
                                .unwrap();
                        rrule_result.dtstart = naive_date.to_string();
                    }
                }
            }

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

            Rule::bymonth_expr => {
                rrule_result.by_month = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(",")
                    .collect();
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

fn generate_rrule_from_json(json: &str) -> RRule {
    serde_json::from_str(json).unwrap()
}

// ToDo : Add validation for checking that the RRULE string was properly extracted from the parser
// by counting ';' in the original rrule string and ':' in the parsed json
fn main() {
    let args: Vec<String> = env::args().collect();
    let s = "FREQ=DAILY;INTERVAL=2;BYDAY=WE;BYMONTH=12;DTSTART=20190327T030000".to_owned();
    let mut rrule_result = RRule::new();
    convert_to_rrule(&mut rrule_result, &s);

    println!("Rrule is {:?}", rrule_result.to_json());
    println!("All iter dates are {:?}", rrule_result.get_all_iter_dates());
    println!("Next dates are {:?}", rrule_result.get_next_iter_dates());
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_rrule, generate_rrule_from_json, RRule};
    use chrono::offset::TimeZone;
    use chrono::{DateTime, Datelike, Duration, Timelike, Utc, Weekday};
    use serde_json::json;
    use std::iter::FromIterator;
    use std::iter::Iterator;

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
            RRuleTestCase{
                rrule_string: "DTSTART;TZID=Australia/Sydney:19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00 AEST","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "DTSTART;TZID=Europe/London:19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00 BST","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "DTSTART=19970714T133000;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "DTSTART=19970714T133000Z;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU",
                expected_flat_json: r#"{"dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "DTSTART=19970714T133000Z;FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000Z",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            },
            RRuleTestCase{
                rrule_string: "FREQ=WEEKLY;INTERVAL=1;BYHOUR=8,12;BYMINUTE=30,45;BYDAY=TU,SU;TZID=Australia/Perth;DTSTART=19970714T133000",
                expected_flat_json: r#"{"tzid":"Australia/Perth","dtstart":"1997-07-14 13:30:00","frequency":"WEEKLY","interval":"1","byHour":["8","12"],"byMinute":["30","45"],"byDay":["TU","SU"]}"#
            }
        ];

        for i in &rrule_test_cases {
            let mut rrule_result = RRule::new();

            convert_to_rrule(&mut rrule_result, i.rrule_string);

            assert_eq!(i.expected_flat_json, rrule_result.to_json())
        }
    }

    #[test]
    fn test_daily_rules_work_as_expected() {
        let mut rrule_result = RRule::new();

        convert_to_rrule(
            &mut rrule_result,
            "FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000",
        );

        assert_eq!(
            vec![
                "2019-04-03 09:01:00".to_owned(),
                "2019-04-10 09:01:00".to_owned(),
                "2019-04-17 09:01:00".to_owned(),
                "2019-04-24 09:01:00".to_owned()
            ],
            rrule_result
                .get_all_iter_dates()
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );

        rrule_result = RRule::new();

        convert_to_rrule(
            &mut rrule_result,
            "FREQ=DAILY;COUNT=20;INTERVAL=3;BYDAY=FR;BYMONTH=11;BYHOUR=10;BYMINUTE=1;BYSECOND=58;DTSTART=20190327T030000",
        );

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
                .get_all_iter_dates()
                .iter()
                .map(|date| date.format("%Y-%m-%d %H:%M:%S").to_string().to_owned())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_we_use_the_count_properly() {
        let mut rrule_result = RRule::new();

        // test we get the right next date
        convert_to_rrule(
            &mut rrule_result,
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        );
        assert_eq!(27, rrule_result.get_next_iter_dates().len())
    }

    #[test]
    fn test_fortnightly_rrules_fall_on_the_day_specified() {
        let mut rrule_result = RRule::new();

        // test case group 1 - implicit by day from dtStart
        convert_to_rrule(
            &mut rrule_result,
            "FREQ=WEEKLY;INTERVAL=2;BYHOUR=0;BYSECOND=48;TZID=Australia/West;DTSTART=20190101T031500",
        );
        let mut iter_dates = rrule_result.get_all_iter_dates();
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Tue, date.weekday());
            assert_eq!(00, date.hour());
            assert_eq!(15, date.minute());
            assert_eq!(48, date.second());
        }

        rrule_result = RRule::new();

        convert_to_rrule(&mut rrule_result, "FREQ=WEEKLY;INTERVAL=2;BYHOUR=17;BYMINUTE=0;TZID=Australia/Sydney;DTSTART=20181122T000003");
        iter_dates = rrule_result.get_all_iter_dates();
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Thu, date.weekday());
            assert_eq!(17, date.hour());
            assert_eq!(00, date.minute());
            assert_eq!(03, date.second());
        }

        rrule_result = RRule::new();
        // test case 2 - explicit by day
        convert_to_rrule(
            &mut rrule_result,
            "FREQ=WEEKLY;INTERVAL=2;BYDAY=FR;TZID=Australia/West;DTSTART=20190101T030000",
        );
        let mut iter_dates = rrule_result.get_all_iter_dates();
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Fri, date.weekday());
            assert_eq!(03, date.hour());
            assert_eq!(00, date.minute());
            assert_eq!(00, date.second());
        }
    }

    #[test]
    fn test_monthly_rrule() {
        let mut rrule_result = RRule::new();
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
        let mut rrule_result = RRule::new();

        // test we get the right next date
        convert_to_rrule(&mut rrule_result, "FREQ=YEARLY;COUNT=2;INTERVAL=1");
        let mut test_start_date = Utc.ymd(2019, 03, 15).and_hms(01, 12, 13);
        assert_eq!(
            test_start_date.with_year(2020).unwrap(),
            rrule_result.get_next_date(test_start_date)
        )
    }

    #[test]
    fn we_can_deserialize_rrule_json_succesfully_1() {
        let mut rrule_expected_1 = RRule::new();

        // test we get the right next date
        convert_to_rrule(&mut rrule_expected_1, "FREQ=YEARLY;COUNT=2;INTERVAL=1");
        let mut rrule_1 = rrule_expected_1.to_json();
        let mut rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref());
        assert_eq!(rrule_actual_1, rrule_expected_1);
    }

    #[test]
    fn we_can_deserialize_rrule_json_succesfully_2() {
        let mut rrule_expected_1 = RRule::new();

        // test we get the right next date
        convert_to_rrule(
            &mut rrule_expected_1,
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        );
        let mut rrule_1 = rrule_expected_1.to_json();
        let mut rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref());
        assert_eq!(rrule_actual_1, rrule_expected_1)
    }
}
