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
        if let Some(track_id) = current_filter.track_id {
            if let Some(eq_value) = track_id.eq {
                condition = condition.add(Column::TrackId.eq(eq_value))
            }
            if let Some(ne_value) = track_id.ne {
                condition = condition.add(Column::TrackId.ne(ne_value))
            }
        }
        if let Some(name) = current_filter.name {
            if let Some(eq_value) = name.eq {
                condition = condition.add(Column::Name.eq(eq_value))
            }
            if let Some(ne_value) = name.ne {
                condition = condition.add(Column::Name.ne(ne_value))
            }
        }
        if let Some(album_id) = current_filter.album_id {
            if let Some(eq_value) = album_id.eq {
                condition = condition.add(Column::AlbumId.eq(eq_value))
            }
            if let Some(ne_value) = album_id.ne {
                condition = condition.add(Column::AlbumId.ne(ne_value))
            }
        }
        if let Some(media_type_id) = current_filter.media_type_id {
            if let Some(eq_value) = media_type_id.eq {
                condition = condition.add(Column::MediaTypeId.eq(eq_value))
            }
            if let Some(ne_value) = media_type_id.ne {
                condition = condition.add(Column::MediaTypeId.ne(ne_value))
            }
        }
        if let Some(genre_id) = current_filter.genre_id {
            if let Some(eq_value) = genre_id.eq {
                condition = condition.add(Column::GenreId.eq(eq_value))
            }
            if let Some(ne_value) = genre_id.ne {
                condition = condition.add(Column::GenreId.ne(ne_value))
            }
        }
        if let Some(composer) = current_filter.composer {
            if let Some(eq_value) = composer.eq {
                condition = condition.add(Column::Composer.eq(eq_value))
            }
            if let Some(ne_value) = composer.ne {
                condition = condition.add(Column::Composer.ne(ne_value))
            }
        }
        if let Some(milliseconds) = current_filter.milliseconds {
            if let Some(eq_value) = milliseconds.eq {
                condition = condition.add(Column::Milliseconds.eq(eq_value))
            }
            if let Some(ne_value) = milliseconds.ne {
                condition = condition.add(Column::Milliseconds.ne(ne_value))
            }
        }
        if let Some(bytes) = current_filter.bytes {
            if let Some(eq_value) = bytes.eq {
                condition = condition.add(Column::Bytes.eq(eq_value))
            }
            if let Some(ne_value) = bytes.ne {
                condition = condition.add(Column::Bytes.ne(ne_value))
            }
        }
        if let Some(unit_price) = current_filter.unit_price {
            if let Some(eq_value) = unit_price.eq {
                condition = condition.add(Column::UnitPrice.eq(eq_value))
            }
            if let Some(ne_value) = unit_price.ne {
                condition = condition.add(Column::UnitPrice.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::tracks::*;
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
    pub async fn unit_price(&self) -> &f64 {
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
    pub unit_price: Option<TypeFilter<f64>>,
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
