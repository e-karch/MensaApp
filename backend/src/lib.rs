#![forbid(unsafe_code)]
#![warn(clippy::pedantic, clippy::nursery, clippy::unwrap_used)]
//! # MensaApp-Backend
//! Backend application for providing and synchronizing meal plan data of the canteens of the Studierendenwerk Karlsruhe. [^1]
//! This application is designed with a transparent three layer model in mind.
//!
//! [^1]: <https://www.sw-ka.de/de/hochschulgastronomie/speiseplan/>

pub mod interface;
pub mod layer;
pub mod startup;
pub mod util;

/// Runs the MensaApp backend.
pub fn run_backend() {
    println!("Hello MensaApp!");
}
