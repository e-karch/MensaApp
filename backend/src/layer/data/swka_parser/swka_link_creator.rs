//! A module for creating links to the html files of the website of the Studierendenwerk Karlsruhe (www.sw-ka.de)
//! The basic structure of a link is as follows:
//!
//! [`BASE_URL`]+[`MENSA_NAMES`]+[`URL_SEPARATOR`]+[`WEEK_SELECTOR`]+number
//!
//! Like this for example:
//!
//! <https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=28>

use chrono::{Datelike, Duration, Local};

use crate::util::Date;

const URL_SEPARATOR: char = '/';
const WEEK_SELECTOR: &str = "?kw=";
const NUMBER_OF_WEEKS_TO_POLL: u32 = 4;

pub struct SwKaLinkCreator {
    base_url: String,
    valid_canteens: Vec<String>,
}

impl SwKaLinkCreator {
    /// Method for creating a [`SwKaLinkCreator`] instance.
    #[must_use]
    pub fn new(base_url: String, valid_canteens: Vec<String>) -> Self {
        Self {
            base_url,
            valid_canteens,
        }
    }

    /// This method creates all urls for the given day.
    /// ## Return
    /// `Vec<String>` containing all urls.
    #[must_use]
    pub fn get_urls(&self, day: Date) -> Vec<String> {
        let calender_week = Self::get_calender_week(day);
        self.valid_canteens
            .iter()
            .map(|mensa| {
                format!(
                    "{BASE_URL}{mensa}{URL_SEPARATOR}{WEEK_SELECTOR}{calender_week}",
                    BASE_URL = self.base_url
                )
            })
            .collect()
    }

    /// This method creates all urls for the next four weeks.
    /// ## Return
    /// `Vec<String>` containing all urls.
    #[must_use]
    pub fn get_all_urls(&self) -> Vec<String> {
        let today = Self::get_todays_date();
        self.get_all_urls_for_next_weeks_from_date(today)
    }

    fn get_all_urls_for_next_weeks_from_date(&self, date: Date) -> Vec<String> {
        (0..NUMBER_OF_WEEKS_TO_POLL)
            .flat_map(|week| Self::get_urls(self, date + Duration::weeks(week.into())))
            .collect()
    }

    fn get_calender_week(day: Date) -> u32 {
        day.iso_week().week()
    }

    fn get_todays_date() -> Date {
        Local::now().date_naive()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use crate::layer::data::swka_parser::test_data::util as test_util;
    use crate::util::Date;

    const URLS_FOR_NEXT_WEEKS: [&str; 28] = [
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_gottesaue/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_moltke/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_x1moltkestrasse/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_erzberger/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_tiefenbronner/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_holzgarten/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_gottesaue/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_moltke/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_x1moltkestrasse/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_erzberger/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_tiefenbronner/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_holzgarten/?kw=29",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_gottesaue/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_moltke/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_x1moltkestrasse/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_erzberger/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_tiefenbronner/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_holzgarten/?kw=30",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_gottesaue/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_moltke/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_x1moltkestrasse/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_erzberger/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_tiefenbronner/?kw=31",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_holzgarten/?kw=31",
    ];

    const URLS_FOR_CURRENT_WEEK: [&str; 7] = [
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_adenauerring/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_gottesaue/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_moltke/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_x1moltkestrasse/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_erzberger/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_tiefenbronner/?kw=28",
        "https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/mensa_holzgarten/?kw=28",
    ];

    #[tokio::test]
    async fn test_get_urls() {
        let date = Date::from_ymd_opt(2023, 7, 10).unwrap();
        let result = test_util::get_creator().get_urls(date);
        assert_eq!(result, URLS_FOR_CURRENT_WEEK);
    }

    #[tokio::test]
    async fn test_get_all_urls() {
        let date = Date::from_ymd_opt(2023, 7, 10).unwrap();
        let result = test_util::get_creator().get_all_urls_for_next_weeks_from_date(date);
        assert_eq!(result, URLS_FOR_NEXT_WEEKS);
    }
}
