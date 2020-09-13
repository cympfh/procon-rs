/// Calendar - Zeller's Theorem
#[derive(Debug, PartialEq, Eq)]
pub enum Day {
    Sat,
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
}
#[derive(Debug, PartialEq, Eq)]
pub enum CalendarType {
    Julian,    // when 4 <= y && y <= 1582
    Gregorian, // when 1582 <= y
}

impl Day {
    fn from(i: usize) -> Day {
        match i {
            0 => Day::Sat,
            1 => Day::Sun,
            2 => Day::Mon,
            3 => Day::Tue,
            4 => Day::Wed,
            5 => Day::Thu,
            6 => Day::Fri,
            _ => panic!("no day"),
        }
    }
}
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Day::Sat => "Saturday",
                Day::Sun => "Sunday",
                Day::Mon => "Monday",
                Day::Tue => "Tuesday",
                Day::Wed => "Wednesday",
                Day::Thu => "Thursday",
                Day::Fri => "Friday",
            }
        )
    }
}

pub fn zeller(y: usize, m: usize, d: usize, ctype: CalendarType) -> Day {
    assert!(m >= 1 && d >= 1);
    let (year, month) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
    let y_up = year % 100;
    let y_down = year / 100;
    let gamma = match ctype {
        CalendarType::Gregorian => 5 * y_down + y_down / 4,
        CalendarType::Julian => 6 * y_down + 5,
    };
    let h = d + (26 * (month + 1) / 10) + y_up + y_up / 4 + gamma;
    Day::from(h % 7)
}

#[cfg(test)]
mod test_zeller {
    #[test]
    fn it_works() {
        use crate::datetime::zeller::*;
        assert_eq!(zeller(2020, 9, 13, CalendarType::Gregorian), Day::Sun);
        assert_eq!(zeller(2019, 12, 25, CalendarType::Gregorian), Day::Wed);
        assert_eq!(zeller(2018, 12, 25, CalendarType::Gregorian), Day::Tue);
        assert_eq!(zeller(2010, 12, 25, CalendarType::Gregorian), Day::Sat);
        assert_eq!(zeller(2001, 12, 25, CalendarType::Gregorian), Day::Tue);
    }
}
