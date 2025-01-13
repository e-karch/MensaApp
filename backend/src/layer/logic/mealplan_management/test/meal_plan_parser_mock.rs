//! This crate contains mocks of [`MealPlanParserMock`] for testing.

use async_trait::async_trait;

use crate::interface::mensa_parser::ParseError;
use crate::{
    interface::mensa_parser::{
        model::{Dish, ParseCanteen, ParseLine},
        MealplanParser,
    },
    util::{
        Additive::Alcohol,
        Allergen::{Er, Pe},
        Date,
        FoodType::Vegan,
        Price,
    },
};

fn get_dish(dish_number: u32) -> Dish {
    Dish {
        name: format!("Autogenerated Dish number {dish_number}"),
        price: Price {
            price_student: 1000,
            price_employee: 1,
            price_guest: 2,
            price_pupil: 3,
        },
        allergens: vec![Pe, Er],
        additives: vec![Alcohol],
        food_type: Vegan,
        env_score: None,
        nutrition_data: None,
    }
}

fn get_dishes(dish_amount: u32) -> Vec<Dish> {
    let mut dishes = Vec::new();
    for i in 1..dish_amount {
        dishes.push(get_dish(i));
    }
    dishes
}

fn get_line(line_number: u32, dish_amount: u32) -> ParseLine {
    ParseLine {
        name: format!("Autogenerated Line number {line_number}"),
        dishes: get_dishes(dish_amount),
        pos: 42_u32,
    }
}

fn get_lines(line_amount: u32, dish_amount: u32) -> Vec<ParseLine> {
    let mut lines = Vec::new();
    for i in 1..line_amount {
        lines.push(get_line(i, dish_amount));
    }
    lines
}

fn get_canteen(canteen_number: u32, line_amount: u32, dish_amount: u32) -> ParseCanteen {
    ParseCanteen {
        name: format!("Autogenerated Canteen number {canteen_number}"),
        lines: get_lines(line_amount, dish_amount),
        pos: 42_u32,
    }
}

fn get_canteens(canteen_amount: u32, line_amount: u32, dish_amount: u32) -> Vec<ParseCanteen> {
    let mut canteens = Vec::new();
    for i in 1..canteen_amount {
        canteens.push(get_canteen(i, line_amount, dish_amount));
    }
    canteens
}

/// Mock of [`MealplanParser`].
pub struct MealPlanParserMock;

#[async_trait]
impl MealplanParser for MealPlanParserMock {
    /// Initiate a parse procedure. Returns a canteen struct containing mealplan data of the given date.
    async fn parse(&self, _day: Date) -> Result<Vec<ParseCanteen>, ParseError> {
        Ok(get_canteens(5, 10, 2))
    }
    /// Initiate a parse procedure. Returns a tuple containing mealplan data of the next four weeks. The tuple contains a canteen struct with the related date.
    async fn parse_all(&self) -> Result<Vec<(Date, Vec<ParseCanteen>)>, ParseError> {
        Ok(vec![(Date::default(), get_canteens(5, 10, 2))])
    }
}
