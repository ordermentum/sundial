#[macro_use]
extern crate pest_derive;

use pest::prec_climber::{Assoc, Operator, PrecClimber};
use serde_json::json;
use std::env;
use serde::Serialize;

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

/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

use pest::iterators::Pair;
use std::collections::HashMap;

/// converts and rrule string to a jsonified response
fn convert_rrule_to_json(
    rrule_string: String,
) {
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

    let parse_result = RRuleParser::parse(Rule::expr, rrule_string.as_str())
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap();

    for mut line in parse_result.into_inner() {
        match line.as_rule() {
            Rule::freq_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str();
                rrule_result.frequency = this_rule.to_string()
            }

            Rule::interval_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str();
                rrule_result.interval = this_rule.to_string()
            }

            Rule::count_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str();
                rrule_result.count = this_rule.to_string()
            }

            Rule::byhour_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();
                rrule_result.by_hour = this_rule
            }

            Rule::byminute_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();
                rrule_result.by_minute = this_rule
            }

            Rule::bysecond_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();;
                rrule_result.by_second = this_rule
            }

            Rule::byday_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();
                rrule_result.by_day = this_rule
            }

            Rule::bymonthday_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();;
                rrule_result.by_month_day = this_rule
            }

            Rule::byyearday_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str().split(",").collect();;
                rrule_result.by_year_day = this_rule
            }
            _ => {}
        }
    }
    println!("Json is {}", serde_json::to_string(&rrule_result).unwrap());
}


// ToDo : Add validation for checking that the RRULE string was properly extracted from the parser
// by counting ';' in the original rrule string and ':' in the parsed json
fn main() {
    let args: Vec<String> = env::args().collect();
    let s = "FREQ=MONTHLY;INTERVAL=1;BYHOUR=9;BYMINUTE=1;BYMONTHDAY=15,27".to_owned();
    convert_rrule_to_json(s)
}


#[cfg(test)]
mod tests {

    use serde_json::json;


}
