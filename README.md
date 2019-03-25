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


Timezones Supported:

```
Africa/Nairobi
Africa/Abidjan
America/Argentina/Catamarca
America/Adak
America/Argentina/Buenos_Aires
America/Argentina/Catamarca
America/Atikokan
America/Argentina/Cordoba
America/Tijuana
America/Indiana/Indianapolis
America/Indiana/Indianapolis
America/Argentina/Jujuy
America/Indiana/Knox
America/Kentucky/Louisville
America/Argentina/Mendoza
America/Toronto
America/Rio_Branco
America/Argentina/Cordoba
America/Tijuana
America/Denver
America/Port_of_Spain
Pacific/Auckland
Asia/Ashgabat
Asia/Kolkata
Asia/Shanghai
Asia/Shanghai
Asia/Dhaka
Asia/Shanghai
Asia/Urumqi
Asia/Kathmandu
Asia/Macau
Asia/Yangon
Asia/Ho_Chi_Minh
Asia/Jerusalem
Asia/Thimphu
Asia/Makassar
Asia/Ulaanbaatar
Atlantic/Faroe
Europe/Oslo
Australia/Sydney
Australia/Sydney
Australia/Lord_Howe
Australia/Sydney
Australia/Darwin
Australia/Brisbane
Australia/Adelaide
Australia/Hobart
Australia/Melbourne
Australia/Perth
Australia/Broken_Hill
America/Rio_Branco
America/Noronha
America/Sao_Paulo
America/Manaus
America/Halifax
America/Winnipeg
America/Regina
America/Toronto
America/Edmonton
America/St_Johns
America/Vancouver
America/Regina
America/Whitehorse
America/Santiago
Pacific/Easter
America/Havana
Africa/Cairo
Europe/Dublin
Europe/London
Europe/Chisinau
Europe/London
Europe/London
Etc/GMT
Etc/GMT
Etc/GMT
Etc/GMT
Asia/Hong_Kong
Atlantic/Reykjavik
Asia/Tehran
Asia/Jerusalem
America/Jamaica
Asia/Tokyo
Pacific/Kwajalein
Africa/Tripoli
America/Tijuana
America/Mazatlan
America/Mexico_City
Pacific/Auckland
Pacific/Chatham
America/Denver
Asia/Shanghai
Pacific/Honolulu
Pacific/Pohnpei
Pacific/Pago_Pago
Pacific/Chuuk
Pacific/Chuuk
Europe/Warsaw
Europe/Lisbon
Asia/Taipei
Asia/Seoul
Asia/Singapore
Europe/Istanbul
Etc/UCT
America/Anchorage
America/Adak
America/Phoenix
America/Chicago
America/Indiana/Indianapolis
America/New_York
Pacific/Honolulu
America/Indiana/Knox
America/Detroit
America/Denver
America/Los_Angeles
Pacific/Pago_Pago
Etc/UTC
Etc/UTC
Europe/Moscow
Etc/UTC
```

### Useful Resources
- [RRule Demo](https://jakubroztocil.github.io/rrule/)
