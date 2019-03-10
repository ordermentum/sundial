extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::prec_climber::{Assoc, Operator, PrecClimber};
use serde_json::json;
use std::env;

use pest::Parser;

#[derive(Parser)]
#[grammar = "rrule.pest"]
struct RRuleParser;


/// error occurred when parsing user input
#[derive(Debug)]
pub struct ParseError {
    pub location: pest::error::InputLocation,
    pub expected: String,
}

/// converts and rrule string to a jsonified response
fn convert_rrule_to_json(
    rrule_string: String,
) {
    let parse_result = RRuleParser::parse(Rule::expr, rrule_string.as_str())
        .expect("unsuccessful parse") // unwrap the parse result
        .next().unwrap();

    for line in parse_result.into_inner() {
        match line.as_rule() {
            Rule::freq_expr => {
                let mut inner_rules = line.into_inner();
                let this_rule = inner_rules.next().unwrap().as_str();
                println!("result is {}", this_rule)
            }

            _ => {}
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = "FREQ=YEARLY;COUNT=6;BYDAY=TU,TH".to_owned();
    convert_rrule_to_json(s)
}
