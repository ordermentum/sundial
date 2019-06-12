#[macro_use]
extern crate pest_derive;

use chrono::prelude::*;
use chrono::{Duration, TimeZone};
use chrono_tz::Tz;
use pest::Parser;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Parser)]
#[grammar = "rrule.pest"]
struct RRuleParser;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RRule<'a> {
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
    /// use sundial::RRule;
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

    #[inline]
    pub fn new_rrule<'b>(
        tzid: String,
        dtstart: String,
        until: String,
        frequency: String,
        count: String,
        interval: String,
        wkst: String,
        by_month: Vec<&'b str>,
        by_hour: Vec<&'b str>,
        by_minute: Vec<&'b str>,
        by_second: Vec<&'b str>,
        by_day: Vec<&'b str>,
        by_month_day: Vec<&'b str>,
        by_year_day: Vec<&'b str>,
    ) -> RRule<'b> {
        return RRule {
            tzid,
            dtstart,
            until,
            frequency,
            count,
            interval,
            wkst,
            by_month,
            by_hour,
            by_minute,
            by_second,
            by_day,
            by_month_day,
            by_year_day,
        };
    }

    // parent function that can get a list of all future iterations based on count
    pub fn get_all_iter_dates(
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
        let start_date = if self.dtstart.is_empty() {
            Utc::now().with_timezone(&timezone)
        } else {
            Utc.datetime_from_str(&self.dtstart, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .with_timezone(&timezone)
        };

        let mut count: i32 = 52; // default count of iterations to build

        let mut until = "";

        // assign default weekstart and reassign if present
        let mut _wkst = "MO";

        if !self.wkst.is_empty() {
            _wkst = &self.wkst
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

    // parent function that can get a list of all future iterations based on count, with the date list staring at or beyond the cutoff_date
    fn get_all_iter_dates_from_cutoff(
        &self,
        count_from_args: &str,
        until_from_args: &str,
        cutoff_date: DateTime<Tz>,
    ) -> Vec<DateTime<Tz>> {
        let timezone: Tz = if self.tzid.is_empty() {
            "UTC".parse().unwrap()
        } else {
            self.tzid.parse().unwrap()
        };

        // we will work under the assumption that the date provided by dtstart parser will always be
        // and we will convert to the required timezone if provided.
        let start_date = if self.dtstart.is_empty() {
            Utc::now().with_timezone(&timezone)
        } else {
            Utc.datetime_from_str(&self.dtstart, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .with_timezone(&timezone)
        };

        let mut count: i32 = 52; // default count of iterations to build

        let mut until = "";

        // assign default weekstart and reassign if present
        let mut _wkst = "MO";

        if !self.wkst.is_empty() {
            _wkst = &self.wkst
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
            while next_dates_list.len().lt(&(count as usize)) {
                next_date = self.get_next_date(next_date);
                if next_date.ge(&cutoff_date) {
                    next_dates_list.push(next_date);
                }
            }
        } else {
            let until_date: DateTime<Tz> = Utc
                .datetime_from_str(&until, "%Y-%m-%d %H:%M:%S")
                .unwrap()
                .with_timezone(&timezone);
            while next_dates_list.len().lt(&(count as usize)) {
                next_date = self.get_next_date(next_date);
                if next_date.gt(&until_date) {
                    break;
                }
                if next_date.ge(&cutoff_date) {
                    next_dates_list.push(next_date);
                }
            }
        }
        next_dates_list
    }

    pub fn get_all_iter_dates_iso8601(
        &self,
        count_from_args: &str,
        until_from_args: &str,
    ) -> Vec<String> {
        convert_datetime_tz_list_to_rfc339(
            self.get_all_iter_dates(count_from_args, until_from_args),
        )
    }

    pub fn get_all_iter_dates_from_today_iso8601(
        &self,
        count_from_args: &str,
        until_from_args: &str,
    ) -> Vec<String> {
        let timezone: Tz = if self.tzid.is_empty() {
            "UTC".parse().unwrap()
        } else {
            self.tzid.parse().unwrap()
        };
        convert_datetime_tz_list_to_rfc339(self.get_all_iter_dates_from_cutoff(
            count_from_args,
            until_from_args,
            Utc::now().with_timezone(&timezone),
        ))
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_json_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    /// Gets a list of next dates that are
    pub fn get_next_iter_dates(
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
    pub fn get_next_date(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let next_date = if self.frequency.eq("YEARLY") {
            self.handle_yearly(start_date)
        } else if self.frequency == "MONTHLY" {
            self.handle_monthly(start_date)
        } else if self.frequency == "WEEKLY" {
            self.handle_weekly(start_date)
        } else if self.frequency == "DAILY" {
            self.handle_daily(start_date)
        } else if self.frequency == "HOURLY" {
            self.handle_hourly(start_date)
        } else if self.frequency == "MINUTELY" {
            self.handle_minutely(start_date)
        } else if self.frequency == "SECONDLY" {
            self.handle_secondly(start_date)
        } else {
            // println!("Given rrule frequency is not supported");
            start_date
        };
        return next_date;
    }

    // set the lower interval time for start date
    fn with_initial_time_intervals(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut start_date_with_intervals = start_date;

        if self.frequency.ne("SECONDLY") {
            let second: u32 = if !self.by_second.is_empty() {
                self.by_second.first().unwrap().parse().unwrap()
            } else {
                start_date.second()
            };
            start_date_with_intervals = start_date_with_intervals.with_second(second).unwrap();
        }

        if self.frequency.ne("SECONDLY") && self.frequency.ne("MINUTELY") {
            let minute: u32 = if !self.by_minute.is_empty() {
                self.by_minute.first().unwrap().parse().unwrap()
            } else {
                start_date.minute()
            };
            start_date_with_intervals = start_date_with_intervals.with_minute(minute).unwrap();
        }

        if self.frequency.ne("SECONDLY")
            && self.frequency.ne("MINUTELY")
            && self.frequency.ne("HOURLY")
        {
            let hour: u32 = if !self.by_hour.is_empty() {
                self.by_hour.first().unwrap().parse().unwrap()
            } else {
                start_date.hour()
            };
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
            next_year += 1;
        }
        next_date
    }

    /// Handles the calculation of next date based on a monthly rule.
    /// Currently supports BYMONTH and BYMONTHDAY params
    fn handle_monthly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut next_date: DateTime<Tz> = self.with_initial_time_intervals(start_date);
        let interval: u32 = self.interval.parse().unwrap_or(1);

        let by_month_day = self.by_month_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        if !by_month_day.is_empty() {
            let by_month_day_u32 = by_month_day.parse::<u32>().unwrap();
            next_date = next_date.with_day(by_month_day_u32).unwrap();
        }

        if !by_month.is_empty() {
            let by_month_u32 = by_month.parse::<u32>().unwrap();
            next_date = next_date.with_month(by_month_u32).unwrap();
            if next_date <= start_date {
                next_date = next_date.with_year(next_date.year() + 1).unwrap();
            }
        }

        // If the calculated next_date is greater than the start date we don't need to add another month
        let start = if next_date.gt(&start_date) { 1 } else { 0 };
        for _i in start..interval {
            next_date = add_month_to_date(next_date);
        }

        next_date
    }

    /// Handles both weekly and special variants of weekly such as [FREQ=WEEKLY;INTERVAL=2;]
    /// which can colloquially evaluate to fortnightly.
    ///
    /// if start_date_with_interval > start_date
    ///     check if start_date_with_intervals is on the same day as today
    ///     if yes, don't add and that's our first dat
    fn handle_weekly(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let mut start_date_with_intervals = self.with_initial_time_intervals(start_date);
        // adjust start_date if it does not start on the start
        let by_day = self
            .by_day
            .first()
            .unwrap_or(&chrono_weekday_to_rrule_byday(start_date.weekday()))
            .to_owned();

        let interval: u32 = self.interval.parse().unwrap_or(1);
        // now adjust the date to match the start day
        let in_future_day = start_date_with_intervals.gt(&start_date)
            && start_date_with_intervals.ordinal() > start_date.ordinal();
        let days_to_adjust =
            self.calculate_weekday_distance(by_day, start_date.weekday(), in_future_day);
        start_date_with_intervals = start_date_with_intervals + Duration::days(days_to_adjust);

        let mut next_date = start_date_with_intervals;
        for _i in 0..interval {
            if start_date_with_intervals.gt(&start_date) {
                break;
            }
            next_date = next_date + Duration::days(7);
        }
        let final_days_to_adjust =
            self.calculate_weekday_distance(by_day, next_date.weekday(), false);
        next_date = next_date + Duration::days(final_days_to_adjust);
        next_date
    }

    fn handle_daily(&self, start_date: DateTime<Tz>) -> DateTime<Tz> {
        let start_date_with_intervals = self.with_initial_time_intervals(start_date);

        let by_day = self.by_day.first().unwrap_or(&"").to_owned();
        let by_month = self.by_month.first().unwrap_or(&"").to_owned();

        let interval: u32 = self.interval.parse().unwrap_or(1);
        let mut next_date = start_date_with_intervals;

        if by_month.is_empty() {
            if by_day.is_empty() {
                if start_date.eq(&next_date) {
                    next_date = next_date + Duration::days(1);
                }
                for _i in 1..interval {
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
        } else if by_day.is_empty() {
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
                    for _i in 0..interval {
                        next_date = next_date + Duration::hours(1)
                    }
                } else {
                    loop {
                        for _i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                        if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                            break;
                        }
                    }
                }
            } else if by_day.is_empty() {
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
        } else {
            loop {
                if by_month.is_empty() {
                    if by_day.is_empty() {
                        for _i in 0..interval {
                            next_date = next_date + Duration::hours(1)
                        }
                    } else {
                        loop {
                            for _i in 0..interval {
                                next_date = next_date + Duration::hours(1)
                            }
                            if chrono_weekday_to_rrule_byday(next_date.weekday()).eq(by_day) {
                                break;
                            }
                        }
                    }
                } else if by_day.is_empty() {
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
            } else if by_day.is_empty() {
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
                } else if by_day.is_empty() {
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
        } else if by_day.is_empty() {
            for _i in 0..interval {
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
pub struct RuleValidationError {
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
pub struct RuleParseError;

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
        converted_dates.push(date.to_rfc3339_opts(SecondsFormat::Secs, false));
    }
    converted_dates
}

/// Converts and rrule string to a rrule struct
pub fn convert_to_rrule(rrule_string: &str) -> Result<RRule, RuleParseError> {
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
                    let tz_split: Vec<&str> = non_validated_dtstart.split(':').collect();
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
                    if non_validated_dtstart.contains('Z') {
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
                if non_validated_until.contains('Z') {
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
                    .split(',')
                    .collect();
            }

            Rule::byhour_expr => {
                rrule_result.by_hour = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect();
            }

            Rule::byminute_expr => {
                rrule_result.by_minute = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect();
            }

            Rule::bysecond_expr => {
                rrule_result.by_second = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect();
            }

            Rule::byday_expr => {
                rrule_result.by_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect();
            }

            Rule::bymonthday_expr => {
                rrule_result.by_month_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
                    .collect();
            }

            Rule::byyearday_expr => {
                rrule_result.by_year_day = line
                    .into_inner()
                    .next()
                    .unwrap()
                    .as_str()
                    .split(',')
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

pub fn validate_rrule(rrule: &RRule) -> Result<(), RuleValidationError> {
    let mut error_string: String = String::from("");
    // validate byhour
    if !rrule.by_hour.is_empty()
        && rrule
            .by_hour
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 23))
    {
        error_string.push_str(&format!(
            "BYHOUR can only be in range 0-23 | Provided value {:?}",
            rrule.by_hour
        ));
    }

    // validate byminute
    if !rrule.by_minute.is_empty()
        && rrule
            .by_minute
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 59))
    {
        error_string.push_str(
            format!(
                "BYMINUTE can only be in range 0-59 | Provided value {:?}",
                rrule.by_minute
            )
            .as_ref(),
        );
    }

    // validate bysecond
    if !rrule.by_second.is_empty()
        && rrule
            .by_second
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 60))
    {
        error_string.push_str(
            format!(
                "BYSECOND can only be in range 0-60 | Provided value {:?}",
                rrule.by_second
            )
            .as_ref(),
        );
    }
    // validate bymonthday
    if !rrule.by_month_day.is_empty()
        && rrule
            .by_month_day
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 31 || x < 1))
    {
        error_string.push_str(
            format!(
                "BYMONTHDAY can only be in range 1-31 | Provided value {:?}",
                rrule.by_month_day
            )
            .as_ref(),
        );
    }

    // validate bymonth
    if !rrule.by_month.is_empty()
        && rrule
            .by_month
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 12 || x < 1))
    {
        error_string.push_str(
            format!(
                "BYMONTH can only be in range 1-12 | Provided value {:?}",
                rrule.by_month
            )
            .as_ref(),
        );
    }

    // validate byyearday
    if !rrule.by_year_day.is_empty()
        && rrule
            .by_year_day
            .iter()
            .map(|x| x.parse::<u32>().unwrap())
            .any(|x| (x > 366 || x < 1))
    {
        error_string.push_str(
            format!(
                "BYYEARDAY can only be in range 1-366 | Provided value {:?}",
                rrule.by_year_day
            )
            .as_ref(),
        );
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

pub fn get_all_iter_dates(
    rrule_string: &str,
    count: &str,
    interval: &str,
) -> Result<Vec<String>, RuleParseError> {
    let rrule_result = convert_to_rrule(rrule_string);
    match rrule_result {
        Ok(rrule) => Ok(rrule.get_all_iter_dates_iso8601(count, interval)),
        Err(_) => Err(RuleParseError),
    }
}

pub fn get_all_iter_dates_from_today(
    rrule_string: &str,
    count: &str,
    interval: &str,
) -> Result<Vec<String>, RuleParseError> {
    let rrule_result = convert_to_rrule(rrule_string);
    match rrule_result {
        Ok(rrule) => Ok(rrule.get_all_iter_dates_from_today_iso8601(count, interval)),
        Err(_) => Err(RuleParseError),
    }
}
