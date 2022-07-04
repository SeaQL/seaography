use crate::graphql::*;
pub use crate::orm::playlist_track::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "PlaylistTrack")]
impl Model {
    pub async fn playlist_id(&self) -> &i32 {
        &self.playlist_id
    }
    pub async fn track_id(&self) -> &i32 {
        &self.track_id
    }
    pub async fn track_tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::tracks::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = TrackTracksFK(self.track_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn playlist_playlists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::playlists::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = PlaylistPlaylistsFK(self.playlist_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "PlaylistTrackFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub playlist_id: Option<TypeFilter<i32>>,
    pub track_id: Option<TypeFilter<i32>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackTracksFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<TrackTracksFK> for OrmDataloader {
    type Value = crate::orm::tracks::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[TrackTracksFK],
    ) -> Result<std::collections::HashMap<TrackTracksFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::tracks::Column::TrackId.as_column_ref())
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
        Ok(crate::orm::tracks::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = TrackTracksFK(model.track_id.clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct PlaylistPlaylistsFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<PlaylistPlaylistsFK> for OrmDataloader {
    type Value = crate::orm::playlists::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[PlaylistPlaylistsFK],
    ) -> Result<std::collections::HashMap<PlaylistPlaylistsFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::playlists::Column::PlaylistId.as_column_ref(),
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
        Ok(crate::orm::playlists::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = PlaylistPlaylistsFK(model.playlist_id.clone());
                (key, model)
            })
            .collect())
    }
}
