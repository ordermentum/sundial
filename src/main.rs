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
use sundial::iter_dates_from_rrule;

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
    let rrule_dates = iter_dates_from_rrule(rrule, count, interval);
    match rrule_dates {
        Ok(rrule) => {
            println!("{:?}", rrule);
        }
        Err(err) => println!("Encountered Rrule parse error"),
    }
}
