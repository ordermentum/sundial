#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate human_panic;

use chrono::prelude::*;
use chrono::{Duration, TimeZone};
use chrono_tz::Tz;
use clap::App;
use pest::Parser;
use serde::de::value::StrDeserializer;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::error::Error;
use std::fmt::{Display, Formatter};
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
    until: String,
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
            until: String::from(""),
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
    fn get_all_iter_dates(
        &self,
        count_from_args: &str,
        until_from_args: &str,
    ) -> Vec<DateTime<Tz>> {
        let timezone: Tz = if self.tzid.is_empty() {
            "UTC".parse().unwrap()
        } else {
            self.tzid.parse().unwrap()
        };

        // we will work under the assumption that the date provided by dtstart parser will always be
        // and we will convert to the required timezone if provided.
        let mut start_date = if self.dtstart.is_empty() {
            Utc::now().with_timezone(&timezone)
        } else {
            Utc.datetime_from_str(&self.dtstart, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .with_timezone(&timezone)
        };

        let mut count: i32 = 52; // default count of iterations to build

        let mut until = "";

        // assign default weekstart and reassign if present
        let mut wkst = "MO";

        if !self.wkst.is_empty() {
            wkst = &self.wkst
        }

        // set count
        if count_from_args.is_empty() {
            if !self.count.is_empty() {
                count = self.count.parse().unwrap()
            }
        } else {
            count = count_from_args.parse::<i32>().unwrap();
        }

        if until_from_args.is_empty() {
            if !self.until.is_empty() {
                until = &self.until;
            }
        } else {
            until = until_from_args;
        }

        let mut next_dates_list: Vec<DateTime<Tz>> = Vec::new();
        let mut next_date = start_date;

        if until.is_empty() {
            for _i in 0..count {
                next_date = self.get_next_date(next_date);
                next_dates_list.push(next_date);
            }
        } else {
            let until_date: DateTime<Tz> = Utc
                .datetime_from_str(&until, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .with_timezone(&timezone);
            for _i in 0..count {
                next_date = self.get_next_date(next_date);
                if next_date.gt(&until_date) {
                    break;
                }
                next_dates_list.push(next_date);
            }
        }
        next_dates_list
    }

    fn get_all_iter_dates_iso8601(
        &self,
        count_from_args: &str,
        until_from_args: &str,
    ) -> Vec<String> {
        convert_datetime_tz_list_to_rfc339(
            self.get_all_iter_dates(count_from_args, until_from_args),
        )
    }

    fn get_next_iter_dates(
        &self,
        count_from_args: &str,
        until_from_args: &str,
    ) -> Vec<DateTime<Tz>> {
        let timezone: Tz = if self.tzid.is_empty() {
            "UTC".parse().unwrap()
        } else {
            self.tzid.parse().unwrap()
        };
        lens_iter_dates(
            self.get_all_iter_dates(count_from_args, until_from_args),
            Utc::now().with_timezone(&timezone),
        )
    }

    // standalone function that gets iterations from a single start date
    fn get_next_date(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut return_date = start_date;

        if self.frequency.eq("YEARLY") {
            return_date = self.handle_yearly(start_date)
        } else if self.frequency == "MONTHLY" {
            return_date = self.handle_monthly(start_date)
        } else if self.frequency == "WEEKLY" {
            return_date = self.handle_weekly(start_date)
        } else if self.frequency == "DAILY" {
            return_date = self.handle_daily(start_date)
        } else if self.frequency == "HOURLY" {
            return_date = self.handle_hourly(start_date);
        } else if self.frequency == "MINUTELY" {
            return_date = self.handle_minutely(start_date);
        } else if self.frequency == "SECONDLY" {
            return_date = self.handle_secondly(start_date);
        } else {
            println!("Given rrule frequency is not supported");
        }
        return_date
    }

    // set the lower interval time for start date
    fn with_initial_time_intervals(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut start_date_with_intervals = start_date;

        if self.frequency.ne("SECONDLY") {
            let mut second: u32 = start_date.second();
            if !self.by_second.is_empty() {
                second = self.by_second.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals = start_date_with_intervals.with_second(second).unwrap();
        }

        if self.frequency.ne("SECONDLY") && self.frequency.ne("MINUTELY") {
            let mut minute: u32 = start_date.minute();
            if !self.by_minute.is_empty() {
                minute = self.by_minute.first().unwrap().parse().unwrap();
            }
            start_date_with_intervals = start_date_with_intervals.with_minute(minute).unwrap();
        }

        if self.frequency.ne("SECONDLY")
            && self.frequency.ne("MINUTELY")
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

    // currently only supports rrules of type: REQ=YEARLY;COUNT=x;INTERVAL=x
    fn handle_yearly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let interval: u32 = self.interval.parse().unwrap_or(1);
        let max_year = 2099;
        let mut next_date = start_date;
        let mut next_year = start_date.year() + 1;
        for _i in 0..interval {
            if next_date.year().lt(&(max_year as i32)) {
                next_date = next_date.with_year(next_year).unwrap()
            }
            next_year = next_year + 1;
        }
        next_date
    }

    /// Handles the calculation of next date based on a monthly rule.
    /// Currently supports BYMONTH and BYMONTHDAY params
    fn handle_monthly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut next_date: DateTime<Tz> = self.with_initial_time_intervals(start_date);
        let interval: u32 = self.interval.parse().unwrap_or(1);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month_day = self.by_month_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        let start_date_day = start_date.day();

        if by_month.is_empty() {
            if by_month_day.is_empty() {
                for _i in 0..interval {
                    next_date = add_month_to_date(next_date);
                }
            } else {
                let by_month_day_u32 = by_month_day.parse::<u32>().unwrap();
                let start_day = start_date.day();
                if start_day < by_month_day_u32 {
                    next_date = next_date.with_day(by_month_day_u32).unwrap();
                    // here we start the interval at 1 since movement by day above counts as an inital monthly move
                    for _i in 1..interval {
                        next_date = add_month_to_date(next_date);
                    }
                } else if start_day > by_month_day_u32 {
                    // move forward a month
                    next_date = add_month_to_date(next_date);
                    next_date = next_date.with_day(by_month_day_u32).unwrap();
                    for _i in 0..interval {
                        next_date = add_month_to_date(next_date);
                    }
                } else if start_day == by_month_day_u32 {
                    if next_date.gt(&start_date) {
                        next_date = add_month_to_date(next_date);
                        next_date = next_date.with_day(by_month_day_u32).unwrap();
                        for _i in 0..interval {
                            next_date = add_month_to_date(next_date);
                        }
                    } else {
                        next_date = next_date.with_day(by_month_day_u32).unwrap();
                        for _i in 0..interval {
                            next_date = add_month_to_date(next_date);
                        }
                    }
                }
            }
        } else {
            loop {
                if by_month_day.is_empty() {
                    for _i in 0..interval {
                        next_date = add_month_to_date(next_date);
                    }
                } else {
                    let by_month_day_u32 = by_month_day.parse::<u32>().unwrap();
                    let start_day = start_date.day();
                    if start_day < by_month_day_u32 {
                        next_date = next_date.with_day(by_month_day_u32).unwrap();
                        for _i in 0..interval {
                            next_date = add_month_to_date(next_date);
                        }
                    } else if start_day > by_month_day_u32 {
                        // move forward a month
                        next_date = add_month_to_date(next_date);
                        next_date = next_date.with_day(by_month_day_u32).unwrap();
                        for _i in 0..interval {
                            next_date = add_month_to_date(next_date);
                        }
                    } else if start_day == by_month_day_u32 {
                        if next_date.ge(&start_date) {
                            next_date = add_month_to_date(next_date);
                            next_date = next_date.with_day(by_month_day_u32).unwrap();
                            for _i in 0..interval {
                                next_date = add_month_to_date(next_date);
                            }
                        } else {
                            next_date = next_date.with_day(by_month_day_u32).unwrap();
                            for _i in 0..interval {
                                next_date = add_month_to_date(next_date);
                            }
                        }
                    }
                }
                if next_date.month().eq(&(by_month.parse::<u32>().unwrap())) {
                    break;
                }
            }
        }
        next_date
    }

    /// Handles both weekly and special variants of weekly such as [FREQ=WEEKLY;INTERVAL=2;]
    /// which can colloquially evaluate to fortnightly.
    fn handle_weekly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut start_date_with_intervals = self.with_initial_time_intervals(start_date);
        // adjust start_date if it does not start on the start
        let by_day = self
            .by_day
            .first()
            .unwrap_or(&chrono_weekday_to_rrule_byday(start_date.weekday()))
            .to_owned();

        // now adjust the date to match the start day
        let in_future = start_date_with_intervals.gt(&start_date);
        let days_to_adjust =
            self.calculate_weekday_distance(by_day, start_date.weekday(), in_future);
        start_date_with_intervals = start_date_with_intervals + Duration::days(days_to_adjust);

        let interval: u32 = self.interval.parse().unwrap_or(1);
        let mut next_date = start_date_with_intervals;
        for _i in 0..interval {
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

    fn handle_daily(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut start_date_with_intervals = self.with_initial_time_intervals(start_date);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        let interval: u32 = self.interval.parse().unwrap_or(1);
        let mut next_date = start_date_with_intervals;

        if by_month.is_empty() {
            if by_day.is_empty() {
                println!("inside empty byday block");
                for _i in 0..interval {
                    next_date = next_date + Duration::days(1);
                }
            } else {
                loop {
                    for _i in 0..interval {
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
                for _i in 0..interval {
                    next_date = next_date + Duration::days(1);
                }
            } else {
                loop {
                    for _i in 0..interval {
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

    fn handle_hourly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut next_date = self.with_initial_time_intervals(start_date);
        let interval: u32 = self.interval.parse().unwrap_or(1);

        let by_hour = self.by_hour.first().unwrap_or(&"").to_owned();
        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        if by_hour.is_empty() {
            if by_month.is_empty() {
                if by_day.is_empty() {
                    for i in 0..interval {
                        next_date = next_date + Duration::hours(1)
                    }
                } else {
                    loop {
                        for i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                        if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                            break;
                        }
                    }
                }
            } else {
                if by_day.is_empty() {
                    for _i in 0..interval {
                        next_date = next_date + Duration::hours(1)
                    }
                } else {
                    loop {
                        for _i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                        if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day)
                            && next_date.month().eq(&(by_month.parse::<u32>().unwrap()))
                        {
                            break;
                        }
                    }
                }
            }
        } else {
            loop {
                if by_month.is_empty() {
                    if by_day.is_empty() {
                        for i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                    } else {
                        loop {
                            for i in 0..interval {
                                next_date = next_date + Duration::hours(1)
                            }
                            if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                                break;
                            }
                        }
                    }
                } else {
                    if by_day.is_empty() {
                        for _i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                    } else {
                        loop {
                            for _i in 0..interval {
                                next_date = next_date + Duration::hours(1)
                            }
                            if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day)
                                && next_date.month().eq(&(by_month.parse::<u32>().unwrap()))
                            {
                                break;
                            }
                        }
                    }
                }
                if next_date.hour().eq(&by_hour.parse::<u32>().unwrap()) {
                    break;
                }
            }
        }
        next_date
    }

    fn handle_minutely(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut next_date = self.with_initial_time_intervals(start_date);
        let interval: u32 = self.interval.parse().unwrap_or(1);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();
        let by_hour = self.by_hour.first().unwrap_or(&"").to_owned();
        let by_minute = self.by_minute.first().unwrap_or(&"").to_owned();

        if by_hour.is_empty() {
            if by_month.is_empty() {
                if by_day.is_empty() {
                    for _i in 0..interval {
                        next_date = next_date + Duration::minutes(1)
                    }
                } else {
                    loop {
                        for _i in 0..interval {
                            next_date = next_date + Duration::minutes(1)
                        }
                        if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                            break;
                        }
                    }
                }
            } else {
                if by_day.is_empty() {
                    for _i in 0..interval {
                        next_date = next_date + Duration::minutes(1)
                    }
                } else {
                    loop {
                        for _i in 0..interval {
                            next_date = next_date + Duration::minutes(1)
                        }
                        if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day)
                            && next_date.month().eq(&(by_month.parse::<u32>().unwrap()))
                        {
                            break;
                        }
                    }
                }
            }
        } else {
            loop {
                if by_month.is_empty() {
                    if by_day.is_empty() {
                        for _i in 0..interval {
                            next_date = next_date + Duration::minutes(1)
                        }
                    } else {
                        loop {
                            for _i in 0..interval {
                                next_date = next_date + Duration::minutes(1)
                            }
                            if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                                break;
                            }
                        }
                    }
                } else {
                    if by_day.is_empty() {
                        for _i in 0..interval {
                            next_date = next_date + Duration::minutes(1)
                        }
                    } else {
                        loop {
                            for _i in 0..interval {
                                next_date = next_date + Duration::minutes(1)
                            }
                            if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day)
                                && next_date.month().eq(&(by_month.parse::<u32>().unwrap()))
                            {
                                break;
                            }
                        }
                    }
                }
                if next_date.hour().eq(&by_hour.parse::<u32>().unwrap()) {
                    break;
                }
            }
        }
        next_date
    }

    fn handle_secondly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut next_date = self.with_initial_time_intervals(start_date);
        let interval: u32 = self.interval.parse().unwrap_or(1);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        if by_month.is_empty() {
            if by_day.is_empty() {
                for _i in 0..interval {
                    next_date = next_date + Duration::seconds(1)
                }
            } else {
                loop {
                    for _i in 0..interval {
                        next_date = next_date + Duration::seconds(1)
                    }
                    if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                        break;
                    }
                }
            }
        } else {
            if by_day.is_empty() {
                for i in 0..interval {
                    next_date = next_date + Duration::seconds(1)
                }
            } else {
                loop {
                    for _i in 0..interval {
                        next_date = next_date + Duration::seconds(1)
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

#[derive(Debug, Clone)]
struct RuleValidationError {
    validation_error_string: String,
}

impl Display for RuleValidationError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(
            f,
            "RRule validation errors encountered: {}",
            self.validation_error_string
        )
    }
}

impl Error for RuleValidationError {
    fn description(&self) -> &str {
        "Encountered Rrule validation errors"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

#[derive(Debug, Clone)]
struct RuleParseError;

impl Display for RuleParseError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "encountered parsing error")
    }
}

impl Error for RuleParseError {
    fn description(&self) -> &str {
        "encountered parsing errors"
    }

    fn cause(&self) -> Option<&std::error::Error> {
        None
    }
}

fn chrono_weekday_to_rrule_byday(weekday: Weekday) -> &'static str {
    return match weekday {
        Weekday::Mon => "MO",
        Weekday::Tue => "TU",
        Weekday::Wed => "WE",
        Weekday::Thu => "TH",
        Weekday::Fri => "FR",
        Weekday::Sat => "SA",
        Weekday::Sun => "SU",
    };
}

/// Adds a month to a given timezone aware `DateTime` type and takes care of any monthly boundaries
fn add_month_to_date(date: DateTime<Tz>) -> DateTime<Tz> {
    let mut date_with_month_added: DateTime<Tz> = date;
    let year = date.year();

    match date.month() {
        2 => {
            // handle leap years
            if year % 4 == 0 {
                if year % 100 == 0 {
                    if year % 400 == 0 {
                        date_with_month_added = date_with_month_added + Duration::days(29);
                    } else {
                        date_with_month_added = date_with_month_added + Duration::days(28);
                    }
                } else {
                    date_with_month_added = date_with_month_added + Duration::days(29);
                }
            } else {
                date_with_month_added = date_with_month_added + Duration::days(28);
            }
        }
        1 | 3 | 5 | 7 | 8 | 10 | 12 => {
            date_with_month_added = date_with_month_added + Duration::days(31);
        }
        4 | 6 | 9 | 11 => {
            date_with_month_added = date_with_month_added + Duration::days(30);
        }
        _ => {
            panic!(
                "Unrecognised month value when adding month to date {:?}",
                date
            );
        }
    }
    date_with_month_added
}

/// Given a `dates_list` of future iteration dates and a `lens_from_date` to look
/// forward from, this function
/// selects the dates that are strictly in the future and returns a modified list
/// with past dates removed.
fn lens_iter_dates(
    dates_list: Vec<DateTime<Tz>>,
    lens_from_date: DateTime<Tz>,
) -> Vec<DateTime<Tz>> {
    let mut lensed_dates_list: Vec<DateTime<Tz>> = Vec::new();
    for date in dates_list.iter() {
        if date.gt(&lens_from_date) {
            // only add dates in the future
            lensed_dates_list.push(date.to_owned())
        }
    }
    lensed_dates_list
}

fn convert_datetime_tz_list_to_rfc339(dates_list: Vec<DateTime<Tz>>) -> Vec<String> {
    let mut converted_dates: Vec<String> = Vec::new();
    for date in dates_list.iter() {
        converted_dates.push(date.to_rfc3339());
    }
    converted_dates
}

/// Converts and rrule string to a rrule struct
fn convert_to_rrule(rrule_string: &str) -> Result<RRule, RuleParseError> {
    let mut rrule_result = RRule::new();

    let parse_result = RRuleParser::parse(Rule::expr, rrule_string)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    for line in parse_result.into_inner() {
        match line.as_rule() {
            Rule::tz_expr => {
                // parse timezone
                let tz_unparsed: String = line.into_inner().next().unwrap().as_str().to_string();
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
                    let non_validated_dtstart: String =
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

            Rule::until_expr_without_tz => {
                let non_validated_until: String =
                    line.into_inner().next().unwrap().as_str().to_string();
                if non_validated_until.contains("Z") {
                    let naive_date =
                        NaiveDateTime::parse_from_str(&non_validated_until, "%Y%m%dT%H%M%SZ")
                            .unwrap();
                    rrule_result.until = naive_date.to_string();
                } else {
                    let naive_date =
                        NaiveDateTime::parse_from_str(&non_validated_until, "%Y%m%dT%H%M%S")
                            .unwrap();
                    rrule_result.until = naive_date.to_string();
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
    match validate_rrule(&rrule_result) {
        Ok(()) => Ok(rrule_result),
        Err(err) => {
            eprintln!("Error encountered: {}", err);
            Err(RuleParseError)
        }
    }
}

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

fn validate_rrule(rrule: &RRule) -> Result<(), RuleValidationError> {
    let mut error_string: String = String::from("");
    // validate byhour
    if !rrule.by_hour.is_empty() {
        if rrule
            .by_hour
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| *x > 23)
            .is_some()
        {
            error_string.push_str(&format!(
                "BYHOUR can only be in range 0-23 | Provided value {:?}",
                rrule.by_hour
            ));
        }
    }

    // validate byminute
    if !rrule.by_minute.is_empty() {
        if rrule
            .by_minute
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| *x > 59)
            .is_some()
        {
            error_string.push_str(
                format!(
                    "BYMINUTE can only be in range 0-59 | Provided value {:?}",
                    rrule.by_minute
                )
                .as_ref(),
            );
        }
    }

    // validate bysecond
    if !rrule.by_second.is_empty() {
        if rrule
            .by_second
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| *x > 60)
            .is_some()
        {
            error_string.push_str(
                format!(
                    "BYSECOND can only be in range 0-60 | Provided value {:?}",
                    rrule.by_second
                )
                .as_ref(),
            );
        }
    }
    // validate bymonthday
    if !rrule.by_month_day.is_empty() {
        if rrule
            .by_month_day
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| (*x > 31 || *x < 1))
            .is_some()
        {
            error_string.push_str(
                format!(
                    "BYMONTHDAY can only be in range 1-31 | Provided value {:?}",
                    rrule.by_month_day
                )
                .as_ref(),
            );
        }
    }

    // validate bymonth
    if !rrule.by_month.is_empty() {
        if rrule
            .by_month
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| (*x > 12 || *x < 1))
            .is_some()
        {
            error_string.push_str(
                format!(
                    "BYMONTH can only be in range 1-12 | Provided value {:?}",
                    rrule.by_month
                )
                .as_ref(),
            );
        }
    }

    // validate byyearday
    if !rrule.by_year_day.is_empty() {
        if rrule
            .by_year_day
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .find(|x| (*x > 366 || *x < 1))
            .is_some()
        {
            error_string.push_str(
                format!(
                    "BYYEARDAY can only be in range 1-366 | Provided value {:?}",
                    rrule.by_year_day
                )
                .as_ref(),
            );
        }
    }

    // validate tzid
    if !rrule.tzid.is_empty() {
        let tz = rrule.tzid.parse::<Tz>();
        if tz.is_err() {
            error_string.push_str(
                format!(
                    "Timezone ID: {:?} is not recognised, please try an IANA recognised tzid",
                    rrule.tzid
                )
                .as_ref(),
            );
        }
    }

    if error_string.is_empty() {
        Ok(())
    } else {
        Err(RuleValidationError {
            validation_error_string: error_string.to_owned(),
        })
    }
}

// ToDo : Add validation for checking that the RRULE string was properly extracted from the parser
// by counting ';' in the original rrule string and ':' in the parsed json
fn main() {
    setup_panic!();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let rrule = matches.value_of("rrule").unwrap_or("");

    if rrule.is_empty() {
        panic!("rrule string cannot be empty, use -h argument to view help");
    }

    let count = matches.value_of("count").unwrap_or("");
    let interval = matches.value_of("until").unwrap_or("");
    let rrule_result = convert_to_rrule(rrule);
    match rrule_result {
        Ok(rrule) => {
            println!("{:?}", rrule.get_all_iter_dates_iso8601(count, interval));
        }
        Err(err) => println!("Encountered Rrule parse error"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{convert_to_rrule, generate_rrule_from_json, RRule, validate_rrule};
    use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc, Weekday};
    use chrono_tz::Etc::UTC;
    use chrono_tz::Tz;
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
            let mut rrule_result = convert_to_rrule(i.rrule_string).unwrap();

            assert_eq!(i.expected_flat_json, rrule_result.to_json())
        }
    }

    #[test]
    fn test_by_hour_validation_works() {
        let rrule = RRule {
            tzid: String::from("Australia/Perth"),
            dtstart: String::from("1997-07-14 13:30:00"),
            until: String::from("2133-04-22 13:35:00"),
            frequency: String::from("WEEKLY"),
            count: String::from(""),
            interval: String::from("1"),
            wkst: String::from(""),
            by_month: Vec::new(),
            by_hour: vec!["8", "28"],
            by_minute: vec!["30", "45"],
            by_second: Vec::new(),
            by_day: vec!["TU", "SU"],
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_by_minute_validation_works() {
        let rrule = RRule {
            tzid: String::from("Australia/Perth"),
            dtstart: String::from("1997-07-14 13:30:00"),
            until: String::from("2133-04-22 13:35:00"),
            frequency: String::from("WEEKLY"),
            count: String::from(""),
            interval: String::from("1"),
            wkst: String::from(""),
            by_month: Vec::new(),
            by_hour: vec!["8", "23"],
            by_minute: vec!["30", "90"],
            by_second: Vec::new(),
            by_day: vec!["TU", "SU"],
            by_month_day: Vec::new(),
            by_year_day: Vec::new(),
        };
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_by_monthday_validation_works() {
        let rrule = RRule {
            tzid: String::from("Australia/Perth"),
            dtstart: String::from("1997-07-14 13:30:00"),
            until: String::from("2133-04-22 13:35:00"),
            frequency: String::from("WEEKLY"),
            count: String::from(""),
            interval: String::from("1"),
            wkst: String::from(""),
            by_month: Vec::new(),
            by_hour: vec!["8", "23"],
            by_minute: vec!["30", "50"],
            by_second: Vec::new(),
            by_day: vec!["TU", "SU"],
            by_month_day: vec!["32"],
            by_year_day: Vec::new(),
        };
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_tzid_validation_works() {
        let rrule = RRule {
            tzid: String::from("Gondwana/BigContinent"),
            dtstart: String::from("1997-07-14 13:30:00"),
            until: String::from("2133-04-22 13:35:00"),
            frequency: String::from("WEEKLY"),
            count: String::from(""),
            interval: String::from("1"),
            wkst: String::from(""),
            by_month: Vec::new(),
            by_hour: vec!["8", "12"],
            by_minute: vec!["30", "33"],
            by_second: Vec::new(),
            by_day: vec!["TU", "SU"],
            by_month_day: vec!["22"],
            by_year_day: Vec::new(),
        };
        assert!(validate_rrule(&rrule).is_err(), true);
    }

    #[test]
    fn test_we_use_the_count_properly() {
        let mut rrule_result = convert_to_rrule(
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        )
        .unwrap();

        assert_eq!(27, rrule_result.get_next_iter_dates("", "").len())
    }

    #[test]
    fn test_until_params_works() {
        let mut rrule_result = convert_to_rrule("FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;DTSTART=20190327T133500;UNTIL=20200612T030000").unwrap();

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
        let mut rrule_result = convert_to_rrule(
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
        let mut rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000;TZID=Australia/Darwin").unwrap();

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
        let mut rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=4;INTERVAL=1;BYDAY=WE;BYHOUR=9;BYMINUTE=1;DTSTART=20190327T030000;TZID=Australia/Brisbane").unwrap();

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
        let mut rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=6;INTERVAL=5;BYDAY=WE;BYHOUR=12;BYMINUTE=52;DTSTART=20190327T030000;TZID=Singapore").unwrap();

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
        let mut rrule_result = convert_to_rrule("FREQ=DAILY;COUNT=20;INTERVAL=3;BYDAY=FR;BYMONTH=11;BYHOUR=10;BYMINUTE=1;BYSECOND=58;DTSTART=20190327T030000").unwrap();

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
    fn test_hourly_rules_work_1() {
        let mut rrule_result =
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
        let mut rrule_result = convert_to_rrule(
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
        let mut rrule_result = convert_to_rrule("FREQ=HOURLY;INTERVAL=3;COUNT=20;BYDAY=TH;DTSTART=20190327T030000;TZID=Australia/Sydney").unwrap();

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
        let mut rrule_result =
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
    fn test_fortnightly_rrules_1() {
        // test case group 1 - implicit by day from dtStart
        let mut rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=0;BYSECOND=48;TZID=Australia/West;DTSTART=20190101T031500").unwrap();
        let mut iter_dates = rrule_result.get_all_iter_dates("", "");
        for date in iter_dates.iter() {
            println!("Checking for date {:?}", date);
            assert_eq!(Weekday::Tue, date.weekday());
            assert_eq!(00, date.hour());
            assert_eq!(15, date.minute());
            assert_eq!(48, date.second());
        }
    }

    #[test]
    fn test_fortnightly_rules_2() {
        let mut rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=17;BYMINUTE=0;TZID=Australia/Sydney;DTSTART=20181122T000003").unwrap();
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
        let mut rrule_result = convert_to_rrule("FREQ=WEEKLY;INTERVAL=2;BYHOUR=17;BYMINUTE=0;TZID=Australia/Sydney;DTSTART=20181122T000003").unwrap();
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
    fn test_monthly_rrule_1() {
        // test we get the right next date
        let mut rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28;DTSTART=20190315T011213;TZID=UTC").unwrap();
        assert_eq!(
            vec!["2019-03-28T09:01:13+00:00".to_owned()],
            rrule_result.get_all_iter_dates_iso8601("", "")
        )
    }

    #[test]
    fn test_monthly_rrule_2() {
        // test we get the right next date
        let mut rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=2;BYMONTHDAY=28;DTSTART=20190402T011213;TZID=Australia/Melbourne").unwrap();
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
        let mut rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=12;BYMONTH=6;DTSTART=20190402T011213;TZID=Australia/Melbourne").unwrap();
        assert_eq!(
            vec![
                "2019-06-02T12:12:13+11:00".to_owned(),
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
        let mut rrule_result = convert_to_rrule("FREQ=MONTHLY;INTERVAL=1;COUNT=12;BYMONTH=11;BYMONTHDAY=12;DTSTART=20190402T011213;TZID=Australia/Sydney").unwrap();
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
    fn we_support_yearly_rules_properly() {
        // test we get the right next date
        let mut rrule_result = convert_to_rrule("FREQ=YEARLY;COUNT=2;INTERVAL=1").unwrap();
        let mut test_start_date = Utc
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
        let mut rrule_expected = convert_to_rrule("FREQ=YEARLY;COUNT=2;INTERVAL=1").unwrap();
        let mut rrule_1 = rrule_expected.to_json();
        let mut rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref()).unwrap();
        assert_eq!(rrule_actual_1, rrule_expected);
    }

    #[test]
    fn we_can_deserialize_rrule_json_succesfully_2() {
        // test we get the right next date
        let mut rrule_expected = convert_to_rrule(
            "FREQ=MONTHLY;COUNT=27;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=28,27",
        )
        .unwrap();
        let mut rrule_1 = rrule_expected.to_json();
        let mut rrule_actual_1 = generate_rrule_from_json(rrule_1.as_ref()).unwrap();
        assert_eq!(rrule_actual_1, rrule_expected)
    }
}
