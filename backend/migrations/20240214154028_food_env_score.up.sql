-- Add up migration script here

CREATE DOMAIN env_rating as smallint CHECK (VALUE > 0);
CREATE DOMAIN env_value as smallint CHECK (VALUE > 0);

CREATE TABLE food_env_score (
    average_rating env_rating NOT NULL,
    co2_rating env_rating NOT NULL,
    co2_value env_value NOT NULL,
    water_rating env_rating NOT NULL,
    water_value env_value NOT NULL,
    animal_welfare_rating env_rating NOT NULL,
    rainforest_rating env_rating NOT NULL,
    max_rating env_rating NOT NULL,
    food_id uuid NOT NULL REFERENCES meal(food_id),
    PRIMARY KEY (food_id)
);