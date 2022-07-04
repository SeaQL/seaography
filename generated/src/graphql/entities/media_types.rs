use crate::graphql::*;
pub use crate::orm::media_types::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "MediaTypes")]
impl Model {
    pub async fn media_type_id(&self) -> &i32 {
        &self.media_type_id
    }
    pub async fn name(&self) -> &Option<String> {
        &self.name
    }
    pub async fn media_type_tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::tracks::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = MediaTypeTracksFK(self.media_type_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "MediaTypesFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub media_type_id: Option<TypeFilter<i32>>,
    pub name: Option<TypeFilter<String>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct MediaTypeTracksFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<MediaTypeTracksFK> for OrmDataloader {
    type Value = Vec<crate::orm::tracks::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[MediaTypeTracksFK],
    ) -> Result<std::collections::HashMap<MediaTypeTracksFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::tracks::Column::MediaTypeId.as_column_ref(),
                )
                .into_simple_expr(),
            ])),
            sea_orm::sea_query::BinOper::In,
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(
                keys.iter()
                    .map(|tuple| {
                        sea_orm::sea_query::SimpleExpr::Values(vec![tuple.0.clone().into()])
                    })
                    .collect(),
            )),
        ));
        use itertools::Itertools;
        Ok(crate::orm::tracks::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = MediaTypeTracksFK(model.media_type_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
