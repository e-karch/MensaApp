//! This interface allows communication with the image hoster.
pub mod model;

use crate::interface::image_hoster::model::ImageMetaData;
use async_trait::async_trait;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, ImageHosterError>;

#[async_trait]
/// This trait provides essential functions, which are necessary for image validation.
pub trait ImageHoster: Sync + Send {
    /// Checks if the given link is valid and provides additional information (ImageMetaData) from the hoster.
    async fn validate_url(&self, url: &str) -> Result<ImageMetaData>;
    /// Checks if an image still exists at the hoster website.
    async fn check_existence(&self, photo_id: &str) -> bool;
    /// Checks whether the licence is acceptable for our purposes.
    async fn check_licence(&self, photo_id: &str) -> bool;
}

/// Enum describing the possible ways, a image hoster request can fail.
#[derive(Debug, Error)]
pub enum ImageHosterError {
    /// Photo not found error
    #[error("the photo id passed was not a valid photo id")]
    PhotoNotFound,
    /// Permission denied error
    #[error("the calling user does not have permission to view the photo")]
    PermissionDenied,
    /// Invalid API Key error
    #[error("the api key passed was not valid or has expired")]
    InvalidApiKey,
    /// Service currently unavailable error
    #[error("the requested service is temporarily unavailable")]
    ServiceUnavailable,
    /// Format "xxx" not found error
    #[error("the requested response format was not found")]
    FormatNotFound(String),
    /// The connection failed to establish error
    #[error("no connection could be established")]
    NotConnected,
    #[error("the html reqwest client creation failed")]
    ClientBuilderFailed(String),
    #[error("some html code couldn't be decoded")]
    DecodeFailed(String),
    #[error("some undefined image hoster error occurred")]
    SomethingWentWrong(String),
}
