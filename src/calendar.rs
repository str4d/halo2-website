use time::{util::is_leap_year, Duration, Month, OffsetDateTime, Weekday};

pub(crate) struct Calendar {
    now: OffsetDateTime,
    days_to_highlight: &'static [Weekday],
}

impl Calendar {
    pub(crate) fn new(days_to_highlight: &'static [Weekday]) -> Self {
        Calendar {
            now: OffsetDateTime::now_utc(),
            days_to_highlight,
        }
    }

    /// Returns the Fluent id for the name of the current month.
    pub(crate) fn month(&self) -> &'static str {
        match self.now.month() {
            Month::January => "month-january",
            Month::February => "month-february",
            Month::March => "month-march",
            Month::April => "month-april",
            Month::May => "month-may",
            Month::June => "month-june",
            Month::July => "month-july",
            Month::August => "month-august",
            Month::September => "month-september",
            Month::October => "month-october",
            Month::November => "month-november",
            Month::December => "month-december",
        }
    }

    /// Returns the Fluent id for the short name of the given weekday.
    pub(crate) fn weekday_short_name(&self, weekday: &u8) -> &'static str {
        match weekday {
            1 => "weekday-short-mon",
            2 => "weekday-short-tue",
            3 => "weekday-short-wed",
            4 => "weekday-short-thu",
            5 => "weekday-short-fri",
            6 => "weekday-short-sat",
            7 => "weekday-short-sun",
            _ => unreachable!(),
        }
    }

    pub(crate) fn first_day_of_month(&self) -> u8 {
        self.now
            .replace_day(1)
            .unwrap()
            .weekday()
            .number_from_monday()
    }

    pub(crate) fn days_this_month(&self) -> u8 {
        match self.now.month() {
            Month::January => 31,
            Month::February => {
                if is_leap_year(self.now.year()) {
                    29
                } else {
                    28
                }
            }
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31,
        }
    }

    pub(crate) fn highlight_day(&self, day: &u8) -> bool {
        self.now
            .replace_day(*day)
            .ok()
            .map_or(false, |d| self.days_to_highlight.contains(&d.weekday()))
    }

    pub(crate) fn next_highlighted_day(&self) -> String {
        let mut tmp = self.now;
        while !self.days_to_highlight.contains(&tmp.weekday()) {
            tmp += Duration::DAY;
        }
        tmp.date().to_string()
    }
}
