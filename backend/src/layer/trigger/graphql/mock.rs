//! This crate contains mocks of [`RequestDataAccess`] and [`Command`] for testing.

use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    interface::{
        api_command::{AuthInfo, Command, Result as CommandResult},
        persistent_data::{
            model::{Canteen, Image, Line, Meal, Side},
            RequestDataAccess, Result as DataResult,
        },
    },
    util::{Date, ReportReason},
};
use crate::util::{Additive, Allergen, MealType, Price};

pub struct RequestDatabaseMock;



#[async_trait]
impl RequestDataAccess for RequestDatabaseMock {
    async fn get_canteen(&self, _id: Uuid) -> DataResult<Option<Canteen>> {
        let canteen = Canteen {
            id: Uuid::default(),
            name: "dummy".to_string(),
        };
        Ok(Option::from(canteen))
    }

    async fn get_canteens(&self) -> DataResult<Vec<Canteen>> {
        let canteen = Canteen {
            id: Uuid::default(),
            name: "dummy".to_string(),
        };
        Ok(vec![canteen])
    }

    async fn get_lines(&self, _canteen_id: Uuid) -> DataResult<Vec<Line>> {
        let line = Line {
            id: Uuid::default(),
            name: "dummy".to_string(),
            canteen_id: Uuid::default(),
        };
        Ok(vec![line])
    }

    async fn get_meal(
        &self,
        _id: Uuid,
        _line_id: Uuid,
        _date: Date,
    ) -> DataResult<Option<Meal>> {
        let meal = Meal {
            id: Uuid::default(),
            name: "dummy".to_string(),
            meal_type: MealType::Vegan,
            price: Price {
                price_student: 0,
                price_employee: 0,
                price_guest: 0,
                price_pupil: 0,
            },
            last_served: NaiveDate::default(),
            next_served: NaiveDate::default(),
            relative_frequency: 0.0,
            rating_count: 0,
            average_rating: 0.0,
            date: Date::from_ymd_opt(2023, 7, 4).expect("HELP!"),
            line_id: Uuid::default(),
        };
        Ok(Option::from(meal))
    }

    async fn get_meals(
        &self,
        _line_id: Uuid,
        _date: Date,
    ) -> DataResult<Vec<Meal>> {
        let meal = Meal {
            id: Uuid::default(),
            name: "dummy".to_string(),
            meal_type: MealType::Vegan,
            price: Price {
                price_student: 0,
                price_employee: 0,
                price_guest: 0,
                price_pupil: 0,
            },
            last_served: NaiveDate::default(),
            next_served: NaiveDate::default(),
            relative_frequency: 0.0,
            rating_count: 0,
            average_rating: 0.0,
            date: Date::from_ymd_opt(2023, 7, 4).expect("HELP!"),
            line_id: Uuid::default(),
        };
        Ok(vec![meal])
    }

    async fn get_sides(&self, _line_id: Uuid, _date: Date) -> DataResult<Vec<Side>> {
        let side = Side {
            id: Uuid::default(),
            name: "dummy".to_string(),
            meal_type: MealType::Vegan,
            price: Price {
                price_student: 0,
                price_employee: 0,
                price_guest: 0,
                price_pupil: 0,
            },
        };
        Ok(vec![side])
    }

    async fn get_visible_images(
        &self,
        _meal_id: Uuid,
        _client_id: Option<Uuid>,
    ) -> DataResult<Vec<Image>> {
        let d1 = Image {
            id: Uuid::default(),
            image_hoster_id: "dummyImageId".to_string(),
            url: String::new(),
            rank: 0.0,
            upvotes: 0,
            downvotes: 0,
        };
        Ok(vec![d1])
    }

    async fn get_personal_rating(
        &self,
        _meal_id: Uuid,
        _client_id: Uuid,
    ) -> DataResult<Option<u32>> {
        Ok(Option::from(42))
    }

    async fn get_personal_upvote(&self, _image_id: Uuid, _client_id: Uuid) -> DataResult<bool> {
        Ok(true)
    }

    async fn get_personal_downvote(&self, _image_id: Uuid, _client_id: Uuid) -> DataResult<bool> {
        Ok(true)
    }

    async fn get_additives(&self, _food_id: crate::util::Uuid) -> DataResult<Vec<Additive>> {
        Ok(vec![Additive::Alcohol, Additive::Sulphur, Additive::Sweetener])
    }

    async fn get_allergens(&self, _food_id: crate::util::Uuid) -> DataResult<Vec<Allergen>> {
        Ok(vec![Allergen::Pi, Allergen::Hf, Allergen::Gl])
    }
}

pub struct CommandMock;

#[async_trait]
impl Command for CommandMock {
    /// Command to report an image. It als gets checked whether the image shall get hidden.
    async fn report_image(
        &self,
        _image_id: Uuid,
        _reason: ReportReason,
        _auth_info: AuthInfo,
    ) -> CommandResult<()> {
        Ok(())
    }

    /// Command to vote up an image. All down-votes of the same user get removed.
    async fn add_image_upvote(&self, _image_id: Uuid, _auth_info: AuthInfo) -> CommandResult<()> {
        Ok(())
    }

    /// Command to vote down an image. All up-votes of the same user get removed.
    async fn add_image_downvote(&self, _image_id: Uuid, _auth_info: AuthInfo) -> CommandResult<()> {
        Ok(())
    }

    /// Command to remove an up-vote for an image.
    async fn remove_image_upvote(
        &self,
        _image_id: Uuid,
        _auth_info: AuthInfo,
    ) -> CommandResult<()> {
        Ok(())
    }

    /// Command to remove an down-vote for an image.
    async fn remove_image_downvote(
        &self,
        _image_id: Uuid,
        _auth_info: AuthInfo,
    ) -> CommandResult<()> {
        Ok(())
    }

    /// Command to link an image to a meal.
    async fn add_image(
        &self,
        _meal_id: Uuid,
        _image_url: String,
        _auth_info: AuthInfo,
    ) -> CommandResult<()> {
        Ok(())
    }

    /// command to add a rating to a meal.
    async fn set_meal_rating(
        &self,
        _meal_id: Uuid,
        _rating: u32,
        _auth_info: AuthInfo,
    ) -> CommandResult<()> {
        Ok(())
    }
}
