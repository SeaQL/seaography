use crate::graphql::*;
pub use crate::orm::tracks::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Tracks")]
impl Model {
    pub async fn track_id(&self) -> &i32 {
        &self.track_id
    }
    pub async fn name(&self) -> &String {
        &self.name
    }
    pub async fn album_id(&self) -> &Option<i32> {
        &self.album_id
    }
    pub async fn media_type_id(&self) -> &i32 {
        &self.media_type_id
    }
    pub async fn genre_id(&self) -> &Option<i32> {
        &self.genre_id
    }
    pub async fn composer(&self) -> &Option<String> {
        &self.composer
    }
    pub async fn milliseconds(&self) -> &i32 {
        &self.milliseconds
    }
    pub async fn bytes(&self) -> &Option<i32> {
        &self.bytes
    }
    pub async fn unit_price(&self) -> &Decimal {
        &self.unit_price
    }
    pub async fn media_type_media_types<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::media_types::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = MediaTypeMediaTypesFK(self.media_type_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
    pub async fn genre_genres<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<crate::orm::genres::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = GenreGenresFK(self.genre_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn album_albums<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<crate::orm::albums::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = AlbumAlbumsFK(self.album_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn track_invoice_items<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::invoice_items::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = TrackInvoiceItemsFK(self.track_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn track_playlist_track<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::playlist_track::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = TrackPlaylistTrackFK(self.track_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "TracksFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub track_id: Option<TypeFilter<i32>>,
    pub name: Option<TypeFilter<String>>,
    pub album_id: Option<TypeFilter<i32>>,
    pub media_type_id: Option<TypeFilter<i32>>,
    pub genre_id: Option<TypeFilter<i32>>,
    pub composer: Option<TypeFilter<String>>,
    pub milliseconds: Option<TypeFilter<i32>>,
    pub bytes: Option<TypeFilter<i32>>,
    pub unit_price: Option<TypeFilter<Decimal>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct MediaTypeMediaTypesFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<MediaTypeMediaTypesFK> for OrmDataloader {
    type Value = crate::orm::media_types::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[MediaTypeMediaTypesFK],
    ) -> Result<std::collections::HashMap<MediaTypeMediaTypesFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::media_types::Column::MediaTypeId.as_column_ref(),
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
        Ok(crate::orm::media_types::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = MediaTypeMediaTypesFK(model.media_type_id.clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct GenreGenresFK(Option<i32>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<GenreGenresFK> for OrmDataloader {
    type Value = crate::orm::genres::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[GenreGenresFK],
    ) -> Result<std::collections::HashMap<GenreGenresFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::genres::Column::GenreId.as_column_ref())
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
        Ok(crate::orm::genres::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = GenreGenresFK(Some(model.genre_id).clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct AlbumAlbumsFK(Option<i32>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AlbumAlbumsFK> for OrmDataloader {
    type Value = crate::orm::albums::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AlbumAlbumsFK],
    ) -> Result<std::collections::HashMap<AlbumAlbumsFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(crate::orm::albums::Column::AlbumId.as_column_ref())
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
        Ok(crate::orm::albums::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = AlbumAlbumsFK(Some(model.album_id).clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackInvoiceItemsFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<TrackInvoiceItemsFK> for OrmDataloader {
    type Value = Vec<crate::orm::invoice_items::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[TrackInvoiceItemsFK],
    ) -> Result<std::collections::HashMap<TrackInvoiceItemsFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::invoice_items::Column::TrackId.as_column_ref(),
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
        Ok(crate::orm::invoice_items::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = TrackInvoiceItemsFK(model.track_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct TrackPlaylistTrackFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<TrackPlaylistTrackFK> for OrmDataloader {
    type Value = Vec<crate::orm::playlist_track::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[TrackPlaylistTrackFK],
    ) -> Result<std::collections::HashMap<TrackPlaylistTrackFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::playlist_track::Column::TrackId.as_column_ref(),
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
                let key = TrackPlaylistTrackFK(model.track_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
