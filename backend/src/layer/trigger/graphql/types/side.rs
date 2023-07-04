use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::{util::{Additive, Allergen, Uuid}, interface::persistent_data::model, layer::trigger::graphql::util::ApiUtil};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Side {
    id: Uuid,
    name: String,
}

#[ComplexObject]
impl Side {
    async fn allergens(&self, ctx: &Context<'_>) -> Result<Vec<Allergen>> {
        let data_access = ctx.get_data_access();
        let allergens = data_access
            .get_allergens(self.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(allergens)
    }

    async fn additives(&self, ctx: &Context<'_>) -> Result<Vec<Additive>> {
        let data_access = ctx.get_data_access();
        let additives = data_access
            .get_additives(self.id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(additives)
    }
}

impl From<model::Side> for Side {
    fn from(value: model::Side) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
