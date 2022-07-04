use crate::graphql::*;
pub use crate::orm::genres::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Genres")]
impl Model {
    pub async fn genre_id(&self) -> &i32 {
        &self.genre_id
    }
    pub async fn name(&self) -> &Option<String> {
        &self.name
    }
    pub async fn genre_tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::tracks::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = GenreTracksFK(self.genre_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "GenresFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub genre_id: Option<TypeFilter<i32>>,
    pub name: Option<TypeFilter<String>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenreTracksFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<GenreTracksFK> for OrmDataloader {
    type Value = Vec<crate::orm::tracks::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[GenreTracksFK],
    ) -> Result<std::collections::HashMap<GenreTracksFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::tracks::Column::GenreId.as_column_ref())
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
                let key = GenreTracksFK(model.genre_id.unwrap().clone());
                (key, model)
            })
            .into_group_map())
    }
}
