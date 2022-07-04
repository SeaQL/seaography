use crate::graphql::*;
pub use crate::orm::playlists::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Playlists")]
impl Model {
    pub async fn playlist_id(&self) -> &i32 {
        &self.playlist_id
    }
    pub async fn name(&self) -> &Option<String> {
        &self.name
    }
    pub async fn playlist_playlist_track<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::playlist_track::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = PlaylistPlaylistTrackFK(self.playlist_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "PlaylistsFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub playlist_id: Option<TypeFilter<i32>>,
    pub name: Option<TypeFilter<String>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PlaylistPlaylistTrackFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<PlaylistPlaylistTrackFK> for OrmDataloader {
    type Value = Vec<crate::orm::playlist_track::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[PlaylistPlaylistTrackFK],
    ) -> Result<std::collections::HashMap<PlaylistPlaylistTrackFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::playlist_track::Column::PlaylistId.as_column_ref(),
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
        Ok(crate::orm::playlist_track::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = PlaylistPlaylistTrackFK(model.playlist_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
