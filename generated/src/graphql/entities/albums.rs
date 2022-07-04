use crate::graphql::*;
pub use crate::orm::albums::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Albums")]
impl Model {
    pub async fn album_id(&self) -> &i32 {
        &self.album_id
    }
    pub async fn title(&self) -> &String {
        &self.title
    }
    pub async fn artist_id(&self) -> &i32 {
        &self.artist_id
    }
    pub async fn album_tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::tracks::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AlbumTracksFK(self.album_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn artist_artists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::artists::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = ArtistArtistsFK(self.artist_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "AlbumsFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub album_id: Option<TypeFilter<i32>>,
    pub title: Option<TypeFilter<String>>,
    pub artist_id: Option<TypeFilter<i32>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlbumTracksFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AlbumTracksFK> for OrmDataloader {
    type Value = Vec<crate::orm::tracks::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AlbumTracksFK],
    ) -> Result<std::collections::HashMap<AlbumTracksFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::tracks::Column::AlbumId.as_column_ref())
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
                let key = AlbumTracksFK(model.album_id.unwrap().clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ArtistArtistsFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<ArtistArtistsFK> for OrmDataloader {
    type Value = crate::orm::artists::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[ArtistArtistsFK],
    ) -> Result<std::collections::HashMap<ArtistArtistsFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::artists::Column::ArtistId.as_column_ref(),
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
        Ok(crate::orm::artists::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = ArtistArtistsFK(model.artist_id.clone());
                (key, model)
            })
            .collect())
    }
}
