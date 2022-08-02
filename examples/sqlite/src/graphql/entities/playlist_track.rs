use sea_orm::prelude::*;
pub fn filter_recursive(root_filter: Option<Filter>) -> sea_orm::Condition {
    let mut condition = sea_orm::Condition::all();
    if let Some(current_filter) = root_filter {
        if let Some(or_filters) = current_filter.or {
            let or_condition = or_filters
                .into_iter()
                .fold(sea_orm::Condition::any(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(or_condition);
        }
        if let Some(and_filters) = current_filter.and {
            let and_condition = and_filters
                .into_iter()
                .fold(sea_orm::Condition::all(), |fold_condition, filter| {
                    fold_condition.add(filter_recursive(Some(*filter)))
                });
            condition = condition.add(and_condition);
        }
        if let Some(playlist_id) = current_filter.playlist_id {
            if let Some(eq_value) = playlist_id.eq {
                condition = condition.add(Column::PlaylistId.eq(eq_value))
            }
            if let Some(ne_value) = playlist_id.ne {
                condition = condition.add(Column::PlaylistId.ne(ne_value))
            }
        }
        if let Some(track_id) = current_filter.track_id {
            if let Some(eq_value) = track_id.eq {
                condition = condition.add(Column::TrackId.eq(eq_value))
            }
            if let Some(ne_value) = track_id.ne {
                condition = condition.add(Column::TrackId.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::playlist_track::*;
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
