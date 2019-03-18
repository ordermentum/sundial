## Sundial 🚧

Sundial is a library written in pure [Rust](https://www.rust-lang.org/) which partially implements the [iCalendar spec](https://tools.ietf.org/html/rfc5545) to support parsing of RRules.

### Current high level features to be supported in this project

- Ability to parse an RRule to a json representation

- Ability to extract an RRule implementation from a given json

- Ability to generate next iteration/iterations given an RRule string

### RFC5545 spec features supported

Since the library is designed purely to support the [RRules section of the spec](https://tools.ietf.org/html/rfc5545#section-3.3.10) at the moment, features will be added iteratively for each type of RRule support:


|            | SECONDLY      | MINUTELY      | HOURLY        | DAILY         | WEEKLY        | FORTNIGHTLY   | MONTHLY       | YEARLY        |
|------------|---------------|---------------|---------------|---------------|---------------|---------------|---------------|---------------|
| COUNT      | <ul><li>- []</li></ul> | <li>- []</li> | <li>- []</li> | <li>- []</li> | <li>- []</li> | <li>- []</li> | <li>- [x] YES</li>| <li>- [x]</li>|
| INTERVAL   | []            | []            | []            | []            | []            | []            | [x]           | [x]           |
| BYMONTH    | []            | []            | []            | []            | []            | []            | [x]           | []            |
| BYWEEKNO   | N/A           | N/A           | N/A           | N/A           | N/A           | N/A           | N/A           | []            |
| BYYEARDAY  | []            | []            | []            | N/A           | N/A           | N/A           | []            | []            |
| BYMONTHDAY | []            | []            | []            | []            | []            | N/A           | [x]           | []            |
| BYDAY      | []            | []            | []            | []            | []            | []            | [x]           | []            |
| BYHOUR     | []            | []            | []            | []            | []            | []            | [x]           | []            |
| BYMINUTE   | []            | []            | []            | []            | []            | []            | [x]           | []            |
| BYSECOND   | []            | []            | []            | []            | []            | []            | [x]           | []            |
| BYSETPOS   | N/S           | N/S           | N/S           | N/S           | N/S           | N/S           | N/S           | N/S           |

* Checked boxes mean the feature is supported
* Unchecked means feature is not supported but will be in future releases
* N/A means the feature is not applicable for the RRule type per the spec
* N/S means the feature will not be support at present by this library
