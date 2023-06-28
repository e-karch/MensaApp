//! This interface allows starting the operations for updating the menu from the the canteen's website.

pub trait MensaParseScheduling {

    /// Initiate the parsing procedure of the canteen-website.
    /// Only parse meals of the current date.
    fn start_update_parsing();

    /// Initiate the parsing procedure of the canteen-website.
    /// Only parse meals for the next four weeks.
    fn start_full_parsing();
}