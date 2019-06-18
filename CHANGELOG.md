# 0.0.4

- Removed too many mutable references improving memory usage and removed all the pesky compiler warnings
- Improved tests
- Added CHANGELOG.md

# 0.0.3

- Fixed bugs in iteration calculation and simplified monthly rules
- Fixed fortnightly rules bug
- Added ability to use current date as DTSTART even when DTSTART is present in the provided string
- Refactored out src and test modules for easier library import

# 0.0.2

- Fixed bugs weekly rules and simplified logic for monthly and yearly rules
- Added ability to pass cli arguments for `count` and `until`

# 0.0.1

Added basic parsing for RRules and results for next expected 52 date counts
