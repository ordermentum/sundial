## Sundial

[![crates.io](https://img.shields.io/crates/v/sundial.svg)](https://crates.io/crates/sundial)
[![crates.io](https://img.shields.io/crates/d/sundial.svg)](https://crates.io/crates/sundial)
[![Build Status](https://badge.buildkite.com/bff0072dc879a668cac85b99f21dedcd21b1ba88016675f5e9.svg)](https://buildkite.com/ordermentum/sundial)

Sundial is a library written in pure [Rust](https://www.rust-lang.org/) which partially implements the [iCalendar spec](https://tools.ietf.org/html/rfc5545) to support parsing of RRules.

### Current high level features to be supported in this project

- Ability to parse an RRule to a json representation

- Ability to extract an RRule implementation from a given json

- Ability to generate next iteration/iterations given an RRule string

### RFC5545 spec features supported

Since the library is designed purely to support the [RRules section of the spec](https://tools.ietf.org/html/rfc5545#section-3.3.10) at the moment, features will be added iteratively for each type of RRule support (note: all frequency types fully support: COUNT, INTERVAL, DTSTART and UNTIL):

| RRULE FREQUENCY | SUPPORTED RRULE Parts                           |
|-----------------|-------------------------------------------------|
| YEARLY          | BYHOUR, BYMINUTE, BYSECOND                      |
| MONTHLY         | BYMONTH, BYMONTHDAY, BYHOUR, BYMINUTE, BYSECOND |
| WEEKLY          | BYDAY, BYHOUR, BYMINUTE, BYSECOND               |
| DAILY           | BYDAY, BYMONTH, BYHOUR, BYMINUTE, BYSECOND      |
| HOURLY          | BYDAY, BYMONTH, BYHOUR, BYMINUTE, BYSECOND      |
| MINUTELY        | BYDAY, BYMONTH, BYHOUR, BYMINUTE, BYSECOND      |
| SECONDLY        | BYDAY, BYMONTH, BYHOUR, BYMINUTE                |


Timezones support is provided via [chrono_tz](https://github.com/chronotope/chrono-tz) and all supported timezones in chrono-tz are supported out of the box. At the moment this library does not support custom timezones.

### Usage:

The packages compiles to a native binary and can be run simply as a simple sys call.

To view help:
```bash
./sundial -h
```

To parse and get the iter dates from an rrule string:

```bash
./sundial --rrule 'Enter your rrule string here'
```

example, running the following:

```bash
./sundial 'FREQ=WEEKLY;INTERVAL=2;COUNT=12;BYHOUR=0;BYMINUTE=0;DTSTART=20190101T030000'
```

gives the results:

```json
[
  "2019-01-15T00:00:00+00:00",
  "2019-01-29T00:00:00+00:00",
  "2019-02-12T00:00:00+00:00",
  "2019-02-26T00:00:00+00:00",
  "2019-03-12T00:00:00+00:00",
  "2019-03-26T00:00:00+00:00",
  "2019-04-09T00:00:00+00:00",
  "2019-04-23T00:00:00+00:00",
  "2019-05-07T00:00:00+00:00",
  "2019-05-21T00:00:00+00:00",
  "2019-06-04T00:00:00+00:00",
  "2019-06-18T00:00:00+00:00"
]
```

We also support specifying count and until as OPTIONAL command line arguments (please not these will override COUNT and UNTIL parts of the provided rrule string if it contains any):

```bashl
./sundial --rrule <rrule_string> -ct 25 -ul 20220123T030000
```

OR

```bashl
./sundial --rrule <rrule_string> --count 25 --until 20220123T030000
```

This will give you the results of the rrule string intervals bounded by the count value of 25 or until 23/12/2022 3 am UTC, whichever comes first.

Note that we currently only support parsing the until value argument as UTC
date.


### Using Date Cutoffs

Sundial supports specifying the fact that the given rrule string might not have an updated `DTSTART` part and that we would like to have the dates from using current date as the cutoff date.

This can be done by adding the optional `--cutoff` cli parameter.

To illustrate,

if we run 15th April 2019 at midnight: 

```bash
./sundial FREQ=WEEKLY;INTERVAL=1;COUNT=3;BYDAY=TU;BYHOUR=23;BYMINUTE=54;BYSECOND=0;TZID=Australia/Melbourne;DTSTART=20180110T034500
```

without specifying the cutoff parameter, sundial will assume the DTSTART (10th January 2019) as the date to start sending results from, giving us:

```
["2018-01-16T23:54:00+11:00", "2018-01-23T23:54:00+11:00", "2018-01-30T23:54:00+11:00"]
```

however, if we specify the cutoff parameter, 

```bash
./sundial FREQ=WEEKLY;INTERVAL=1;COUNT=3;BYDAY=TU;BYHOUR=23;BYMINUTE=54;BYSECOND=0;TZID=Australia/Melbourne;DTSTART=20180110T034500 --cutoff
```

sundial will keep on calculating until we start off on a date that is greater than or equal to current run date (in this case 15th April 2019 at midnight) and send a count of dates starting from that date, resulting in:

```
["2019-04-16T23:54:00+10:00", "2019-04-23T23:54:00+10:00", "2019-04-30T23:54:00+10:00"]
```

------------------------------------------------------------

### Running tests
To run the full test suite:
```
$ cargo test --all
```

from the repository root.


------------------------------------------------------------
### Useful Resources
- [RRule Demo](https://jakubroztocil.github.io/rrule/)
