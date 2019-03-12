# sundial

Sundial is an implementation of [RFC-5545](https://tools.ietf.org/html/rfc5545) in Rust.

Three main features to be supported in the first pass of this library:

- RRule.toJSON/RRule.fromJSON - parses a rrule string and returns an JSON object representation of the rrule

- RRule.iterator(startDate: timestamp, x: integer) = the next x occurrences of the rrule from the startDate

- getNext - returns the next occurance of the rrule.
