//! Date arithmetic for the picker's timestamp labels.

use crate::{epoch_secs_to_date, is_leap_year};

const DAY: u64 = 60 * 60 * 24;

#[test]
fn leap_year_rules_follow_the_gregorian_calendar() {
    assert!(is_leap_year(2024));
    assert!(is_leap_year(2020));
    assert!(!is_leap_year(2023));
    assert!(!is_leap_year(2021));
    // Divisible by 100 but not 400: not a leap year.
    assert!(!is_leap_year(1900));
    assert!(!is_leap_year(2100));
    // Divisible by 400: still a leap year.
    assert!(is_leap_year(2000));
    assert!(is_leap_year(1600));
}

#[test]
fn epoch_itself_is_the_first_of_january_1970() {
    assert_eq!(epoch_secs_to_date(0), (1, 1));
}

#[test]
fn second_day_of_the_epoch_month() {
    assert_eq!(epoch_secs_to_date(DAY), (2, 1));
}

#[test]
fn month_boundaries_roll_over() {
    assert_eq!(epoch_secs_to_date(31 * DAY), (1, 2)); // 1 Feb 1970
    assert_eq!(epoch_secs_to_date(59 * DAY), (1, 3)); // 1 Mar 1970
    assert_eq!(epoch_secs_to_date(30 * DAY), (31, 1)); // 31 Jan 1970
}

#[test]
fn non_leap_year_has_365_days() {
    assert_eq!(epoch_secs_to_date(365 * DAY), (1, 1)); // 1 Jan 1971
}

#[test]
fn years_accumulate_correctly_across_a_leap_year() {
    // 1970-01-01 to 2000-01-01 is 30 years containing 7 leap days.
    let days = 30 * 365 + 7;
    assert_eq!(epoch_secs_to_date(days * DAY), (1, 1));
    // 2000 is a leap year, so 29 Feb 2000 exists and 1 Mar is day 31+29.
    assert_eq!(epoch_secs_to_date((days + 31 + 29) * DAY), (1, 3));
    assert_eq!(epoch_secs_to_date((days + 31 + 28) * DAY), (29, 2)); // 29 Feb 2000
}

#[test]
fn far_future_dates_do_not_become_absurd() {
    // Guards the year-by-year walk: it must terminate and stay in range.
    let (day, month) = epoch_secs_to_date(2_000_000_000_000);
    assert!((1..=31).contains(&day), "day out of range: {}", day);
    assert!((1..=12).contains(&month), "month out of range: {}", month);
}
