use crate::interface::image_api::{ImageApiError, Result};
use crate::layer::data::image_api::json_structs::{EncodedRequestJson, SafeSearchResponseJson};

pub fn create_json_request() -> Result<EncodedRequestJson> {
    todo!()
}

pub fn parse_valid_response() -> Result<SafeSearchResponseJson> {
    todo!()
}

fn parse_error_response() -> ImageApiError {
    todo!()
}
