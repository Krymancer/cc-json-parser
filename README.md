# JSON Parser

This is a json parser that I code for a [coding challenge](https://codingchallenges.fyi/challenges/challenge-json-parser/) is writed in rust to be blazing fast TM (jk)

## Test Suite

I tested the parser with [JSON_checker](https://www.json.org/JSON_checker/) by json.org.
Are 36 tests with names {pass/fail}{number}.json and as you would guess the parser needs to fail any of the fail files and correctly parse the others.

I did my best and had to cover some edge cases (I know that the code is not very beateful ok) like leading zero numbers, the depth in arrays/objects, scientific notation numbers and of course scaping unicode sequences (urgh).

Besides the json.org test suite the coding challeng has more 11 tests and I create more one to be sure.
The 12 frists tests (in the file `src/tests.rs`) are these one and they test not only either the parser was able to parse the files or not but correctly parse the values.

I cloud write a script to do the same with the 36 tests for json.org but I really tired of working on this project.

## Refactoring

I see room for a lot of improvment, I will revisit this project (famous last words) and do this later :p
