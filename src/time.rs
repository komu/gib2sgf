#[derive(Eq, PartialEq, Debug)]
pub struct LocalDate {
    year: i16,
    month: i8,
    day: i8,
}

impl LocalDate {
    pub fn ymd(year: i16, month: i8, day: i8) -> Option<LocalDate> {
        // TODO: argument validation
        Some(LocalDate { year, month, day })
    }

    pub fn iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
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
