extern crate chrono;

use chrono::prelude::*;
use std::env;
use fdate::RepublicanDate;

const DEFAULT_FMT: &str = "%A, %d %B an %Y (%J). %Hh %Mm %Ss";

const VERSION: &str = env!("CARGO_PKG_VERSION");
const BINARY: &str = env!("CARGO_BIN_NAME");

fn main() {
    let datetime = Local::now();
    let jd = datetime_to_jd(datetime);
    let date = jd_to_french_republican(jd);
    let time = datetime_to_decimal_time(datetime);

    let republican = RepublicanDate::new(date, time);

    if env::args().len() == 2 {
        let arg = env::args().nth(1).unwrap();
        if Some('+') == arg.chars().nth(0) {
            if let Some(fmt) = arg.get(1..) {
                println!("{}", republican.format_str(fmt));
            } else {
                usage();
            }
        } else {
            usage();
        }
    } else if env::args().len() > 2 {
        usage();
    } else {
        println!("{}", republican.format_str(DEFAULT_FMT));
    }

}

fn usage() {
    eprintln!("{} ({})", BINARY, VERSION);
    eprintln!("usage: {} [+format]", env::args().nth(0).unwrap());
}

// Jean Meeus, Astronomical Algorithms. (1991) pp. 60-61
fn datetime_to_jd(datetime: DateTime<Local>) -> f64 {
    let (y, m) = if datetime.month() > 2 {
        (datetime.year() as f64, datetime.month() as f64)
    } else {
        ((datetime.year() - 1) as f64, (datetime.month() + 12) as f64)
    };

    let a = (y / 100.0).trunc();
    let b = 2.0 - a + (a / 4.0).trunc();

    let d = datetime.day() as f64
        + (datetime.hour() as f64) / 24.0
        + (datetime.minute() as f64) / 1440.0
        + (datetime.second() as f64) / 86400.0;

    let jd = (365.25 * (y + 4716.0)).trunc() + (30.6001 * (m + 1.0)).trunc()
        + d + b - 1524.5;

    return jd;
}

// Returns (year, month, day)
// Equations: Hatcher (1985)
// Coefficients: Parisot (1986)
// For the sake of clarity variables are the same as in the paper
#[allow(non_snake_case)]
fn jd_to_french_republican(jd: f64) -> (i64, u8, u8) {
    // Define parameters
    let y = 6504.0;
    let j = 111.0;
    let m = 1.0;
    let n = 13.0;
    let r = 4.0;
    let p = 1461.0;
    let v = 3.0;
    let s = 30.0;

    // Perform the calculations
    let J_prime = jd + j;
    let Y_prime = ((r * J_prime + v) / p).trunc();
    let T_prime = (((r * J_prime + v) % p) / r).trunc();
    let M_prime = (T_prime / s).trunc();
    let D_prime = T_prime % s;

    let D = D_prime + 1.0;
    let M = (M_prime + m - 1.0) % n + 1.0;
    let Y = (Y_prime - y + (n + m - 1.0 - M) / n).trunc();

    return (Y as i64, M as u8, D as u8);
}

// Returns (hour, minute, second)
// 10 hours in a day, 100 minutes in an hour, 100 seconds in a minute
fn datetime_to_decimal_time(dt: DateTime<Local>) -> (u8, u8, u8) {
    let x = (dt.hour() as f64 * 3600.0 + dt.minute() as f64 * 60.0
             + dt.second() as f64) / 86400.0;

    let h = x * 10.0;
    let m = (x * 1000.0) % 100.0;
    let s = (x * 100000.0) % 100.0;

    return (h as u8, m as u8, s as u8);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn datetime_to_jd_test() {
        let datetime = Local.ymd(1999, 1, 1).and_hms(0, 0, 0);
        let jd = datetime_to_jd(datetime);
        assert_eq!(jd, 2451179.5);
    }

    #[test]
    fn republican_calendar() {
        let datetime = Local.ymd(1792, 9, 22).and_hms(0, 0, 0);
        let jd = datetime_to_jd(datetime);
        let republican = jd_to_french_republican(jd);
        assert_eq!(republican, (1, 1, 1));
    }

    #[test]
    fn decimal_time() {
        let date = Local.ymd(1999, 1, 1);
        let time1 = datetime_to_decimal_time(date.and_hms(21, 36, 0));
        assert_eq!(time1, (9, 0, 0));

        let time2 = datetime_to_decimal_time(date.and_hms(21, 46, 43));
        assert_eq!(time2, (9, 7, 44));
    }
}

