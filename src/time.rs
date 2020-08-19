#[derive(Eq, PartialEq, Debug)]
pub struct LocalDate {
    year: u16,
    month: u8,
    day: u8,
}

impl LocalDate {
    pub fn ymd(year: u16, month: u8, day: u8) -> Result<LocalDate, String> {
        if month >= 1 && month <= 12 && day >= 1 && day <= days_in_month(month, year) {
            Ok(LocalDate { year, month, day })
        } else {
            Err(format!("invalid month {:04}-{:02}-{:02}", year, month, day))
        }
    }

    pub fn iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn days_in_month(month: u8, year: u16) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => if is_leap_year(year) { 29 } else { 28 },
        _ => 30
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 400 == 0 || (year % 100 != 0 && year % 4 == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_date_format() {
        let date = LocalDate::ymd(2020, 4, 7).unwrap();

        assert_eq!(date.iso_string(), "2020-04-07");
    }
}
