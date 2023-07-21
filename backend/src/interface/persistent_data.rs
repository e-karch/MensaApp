//! The interfaces specified here allow access to data stored in a persistent datastore like a database.
pub mod model;

use crate::interface::persistent_data::model::{
    ApiKey, Canteen, Image, ImageInfo, Line, Meal, Side,
};
use crate::util::{Additive, Allergen, Date, MealType, Price, ReportReason, Uuid};
use async_trait::async_trait;
use std::num::TryFromIntError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, DataError>;

/// Enumerations for possible data request faults
#[derive(Debug, Error)]
pub enum DataError {
    /// Requested data does not exist
    #[error("the requested item could not be found in the database")]
    NoSuchItem,
    /// Error occurred during data request or an internal connection fault
    #[error("internal error ocurred: {0}")]
    InternalError(#[from] sqlx::Error),
    ///
    #[error("error converting type: {0}")]
    TypeConversionError(#[from] TryFromIntError),
}

#[async_trait]
/// An interface for checking relations and inserting data structures. The MealPlanManagement component uses this interface for database access.
pub trait MealplanManagementDataAccess: Send + Sync {
    /// Removes all relations to the meal plan at the given date and the given canteen.
    /// Without removing changes in the meal plan couldn't be updated.
    async fn dissolve_relations(&self, canteen_id: Uuid, date: Date) -> Result<()>;
    /// Determines the canteen with the most similar name.
    /// Returns the uuid to the similar canteen.
    async fn get_similar_canteen(&self, similar_name: &str) -> Result<Option<Uuid>>;
    /// Determines the line with the most similar name.
    /// Returns the uuid to the similar line.
    async fn get_similar_line(&self, similar_name: &str) -> Result<Option<Uuid>>;
    /// Determines the meal with the most similar name, identical allergens and identical additives.
    /// Returns the uuid to the similar meal.
    async fn get_similar_meal(
        &self,
        similar_name: &str,
        allergens: &[Allergen],
        additives: &[Additive],
    ) -> Result<Option<Uuid>>;
    /// Determines the side with the most similar name, identical allergens and identical additives.
    /// Returns the uuid to the similar canteen.
    async fn get_similar_side(
        &self,
        similar_name: &str,
        allergens: &[Allergen],
        additives: &[Additive],
    ) -> Result<Option<Uuid>>;

    /// Updates an existing canteen entity in the database.
    /// Returns the canteen uuid.
    async fn update_canteen(&self, uuid: Uuid, name: &str) -> Result<Uuid>;
    /// Updates an existing line entity in the database.
    /// Returns the line uuid.
    async fn update_line(&self, uuid: Uuid, name: &str) -> Result<Uuid>;
    /// Updates an existing meal entity in the database.
    async fn update_meal(&self, uuid: Uuid, name: &str) -> Result<()>;
    /// Adds a meal into the meal plan with the given params.
    async fn add_meal_to_plan(
        &self,
        canteen_id: Uuid,
        date: Date,
        meal_id: Uuid,
        price: Price,
    ) -> Result<()>;
    /// Updates an existing side entity in the database.
    async fn update_side(&self, uuid: Uuid, name: &str) -> Result<()>;
    /// Adds a side into the meal plan with the given params.
    async fn add_side_to_plan(
        &self,
        canteen_id: Uuid,
        date: Date,
        side_id: Uuid,
        price: Price,
    ) -> Result<()>;

    /// Adds a new canteen entity to the database.
    /// Returns uuid of the new canteen.
    async fn insert_canteen(&self, name: &str) -> Result<Uuid>;
    /// Adds a new line entity to the database.
    /// Returns uuid of the new line.
    async fn insert_line(&self, name: &str) -> Result<Uuid>;
    /// Adds a new meal entity to the database and creates a meal plan entry for the given day.
    async fn insert_meal(
        &self,
        name: &str,
        meal_type: MealType,
        price: Price,
        allergens: &[Allergen],
        additives: &[Additive],
        canteen_id: Uuid,
        next_served: Date,
    ) -> Result<()>;
    /// Adds a new side entity to the database and creates a meal plan entry for the given day.
    async fn insert_side(
        &self,
        name: &str,
        meal_type: MealType,
        price: Price,
        allergens: &[Allergen],
        additives: &[Additive],
        canteen_id: Uuid,
        next_served: Date,
    ) -> Result<()>;
}

#[async_trait]
/// An interface for image related data. The ImageReview component uses this interface for database access.
pub trait ImageReviewDataAccess {
    /// Returns the first n images sorted by rank which are related to an meal served at the given day.
    async fn get_n_images_by_rank_date(&self, n: u32, date: Date) -> Result<Vec<Image>>;
    /// Returns the first n images sorted by rank which are related to an meal served in the next week or which were not validated last week.
    async fn get_n_images_next_week_by_rank_not_checked_last_week(
        &self,
        n: u32,
    ) -> Result<Vec<Image>>;
    /// Returns the first n images sorted by the date of the last check (desc) which were not validated in the last week.
    async fn get_n_images_by_last_checked_not_checked_last_week(
        &self,
        n: u32,
    ) -> Result<Vec<Image>>;
    /// Removes an image with all relations from the database.
    async fn delete_image(&self, id: Uuid) -> Result<bool>;
    /// Marks all images identified by the given uuids as checked.
    async fn mark_as_checked(&self, ids: Vec<Uuid>) -> Result<()>;
}

#[async_trait]
/// An interface for graphql mutation data manipulation. The Command component uses this interface for database access.
pub trait CommandDataAccess {
    /// Returns the ImageInfo struct of image.
    async fn get_image_info(&self, image_id: Uuid) -> Result<ImageInfo>;
    /// Marks an image as hidden. Hidden images cant be seen by users.
    async fn hide_image(&self, image_id: Uuid) -> Result<()>;
    /// Saves an image report
    async fn add_report(&self, image_id: Uuid, client_id: Uuid, reason: ReportReason)
        -> Result<()>;
    /// Adds an upvote to the given image. An user can only down- or upvote an image.
    async fn add_upvote(&self, image_id: Uuid, user_id: Uuid) -> Result<()>;
    /// Adds a downvote to the given image. An user can only down- or upvote an image.
    async fn add_downvote(&self, image_id: Uuid, user_id: Uuid) -> Result<()>;
    /// Removes an upvote from the given image.
    async fn remove_upvote(&self, image_id: Uuid, user_id: Uuid) -> Result<()>;
    /// Removes a downvote from the given image.
    async fn remove_downvote(&self, image_id: Uuid, user_id: Uuid) -> Result<()>;
    /// Adds an image link to the database. The image will be related to the given meal.
    async fn link_image(
        &self,
        meal_id: Uuid,
        user_id: Uuid,
        image_hoster_id: String,
        url: String,
    ) -> Result<()>;
    /// Adds a rating to the database. The rating will be related to the given meal and the given user.
    async fn add_rating(&self, meal_id: Uuid, user_id: Uuid, rating: u32) -> Result<()>;
    /// Loads all api_keys from the database.
    async fn get_api_keys(&self) -> Result<Vec<ApiKey>>;
}

#[async_trait]
/// An interface for graphql query data. The GraphQL component uses this interface for database access.
pub trait RequestDataAccess {
    /// Returns the canteen from the database.
    async fn get_canteen(&self, id: Uuid) -> Result<Option<Canteen>>;
    /// Returns all canteens from the database.
    async fn get_canteens(&self) -> Result<Vec<Canteen>>;
    /// Returns the line from the database.
    async fn get_line(&self, id: Uuid) -> Result<Option<Line>>;
    /// Returns all lines of a canteen from the database.
    async fn get_lines(&self, canteen_id: Uuid) -> Result<Vec<Line>>;
    /// Returns the meal related to all the params.
    async fn get_meal(&self, id: Uuid, line_id: Uuid, date: Date) -> Result<Option<Meal>>;
    /// Returns all meals related to all the params. Null is returned when there is not any information available yet.
    async fn get_meals(&self, line_id: Uuid, date: Date) -> Result<Option<Vec<Meal>>>;
    /// Returns all sides of a line at the given day from the database.
    async fn get_sides(&self, line_id: Uuid, date: Date) -> Result<Vec<Side>>;
    /// Returns all images, which are related to the given user or meal. Images reported by the user will not be returned.
    async fn get_visible_images(
        &self,
        meal_id: Uuid,
        client_id: Option<Uuid>,
    ) -> Result<Vec<Image>>;
    /// Returns the rating done by the given user for the given meal.
    async fn get_personal_rating(&self, meal_id: Uuid, client_id: Uuid) -> Result<Option<u32>>;
    /// Checks if the given image got an upvote by the given user
    async fn get_personal_upvote(&self, image_id: Uuid, client_id: Uuid) -> Result<bool>;
    /// Checks if the given image got an downvote by the given user
    async fn get_personal_downvote(&self, image_id: Uuid, client_id: Uuid) -> Result<bool>;
    /// Returns all additives related to the given food_id (food_id can be a meal_id or side_id).
    async fn get_additives(&self, food_id: Uuid) -> Result<Vec<Additive>>;
    /// Returns all allergens related to the given food_id (food_id can be a meal_id or side_id).
    async fn get_allergens(&self, food_id: Uuid) -> Result<Vec<Allergen>>;
}
