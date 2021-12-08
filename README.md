# fdate

Date util that outputs French Republican Calendar Date and Decimal time to
stdout.

TODO: Add date input to get other dates (like ddate or date)

## Formatting

* `%%` - literal %
* `%A` - name of weekday (decade-day?) or name of day during Sansculottides
* `%B` - name of month
* `%d` - number of day in month (0..30)
* `%H` - decimal hour (0..9)
* `%j` - day of year (1..366)
* `%J` - day name according the the "rural calendar" (nothing during
  Sansculottides)
* `%m` - month number (1..13)
* `%M` - decimal minute (0..99)
* `%n` - newline
* `%S` - decimal second (0..99)
* `%t` - tab
* `%u` - week (decade) number (1..3, 0 during Sansculottides)
* `%W` - week (decade) number in roman numerals
* `%y` - year (can be negative)
* `%Y` - year in roman numerals (can be negative)

## Acknowledgments

### Crates

* chrono by Kang Seonghoon and contributors.

### Other

* _Astronomical Algorithms_ by Jean Meeus (1991)
* _Generalized Equations for Julian Day Numbers and Calendar Dates_ by D. A.
  Hatcher (1985)
* _Additif to 'Generalized Equations for Julian Day Numbers and Calendar
  Dates'_ by J. P. Parisot (1986)
* `tex.web` by Don Knuth (1982)

