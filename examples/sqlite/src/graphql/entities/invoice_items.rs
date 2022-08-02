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
        if let Some(invoice_line_id) = current_filter.invoice_line_id {
            if let Some(eq_value) = invoice_line_id.eq {
                condition = condition.add(Column::InvoiceLineId.eq(eq_value))
            }
            if let Some(ne_value) = invoice_line_id.ne {
                condition = condition.add(Column::InvoiceLineId.ne(ne_value))
            }
        }
        if let Some(invoice_id) = current_filter.invoice_id {
            if let Some(eq_value) = invoice_id.eq {
                condition = condition.add(Column::InvoiceId.eq(eq_value))
            }
            if let Some(ne_value) = invoice_id.ne {
                condition = condition.add(Column::InvoiceId.ne(ne_value))
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
        if let Some(unit_price) = current_filter.unit_price {
            if let Some(eq_value) = unit_price.eq {
                condition = condition.add(Column::UnitPrice.eq(eq_value))
            }
            if let Some(ne_value) = unit_price.ne {
                condition = condition.add(Column::UnitPrice.ne(ne_value))
            }
        }
        if let Some(quantity) = current_filter.quantity {
            if let Some(eq_value) = quantity.eq {
                condition = condition.add(Column::Quantity.eq(eq_value))
            }
            if let Some(ne_value) = quantity.ne {
                condition = condition.add(Column::Quantity.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::invoice_items::*;
#[async_graphql::Object(name = "InvoiceItems")]
impl Model {
    pub async fn invoice_line_id(&self) -> &i32 {
        &self.invoice_line_id
    }
    pub async fn invoice_id(&self) -> &i32 {
        &self.invoice_id
    }
    pub async fn track_id(&self) -> &i32 {
        &self.track_id
    }
    pub async fn unit_price(&self) -> &f64 {
        &self.unit_price
    }
    pub async fn quantity(&self) -> &i32 {
        &self.quantity
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
    pub async fn invoice_invoices<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::invoices::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = InvoiceInvoicesFK(self.invoice_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "InvoiceItemsFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub invoice_line_id: Option<TypeFilter<i32>>,
    pub invoice_id: Option<TypeFilter<i32>>,
    pub track_id: Option<TypeFilter<i32>>,
    pub unit_price: Option<TypeFilter<f64>>,
    pub quantity: Option<TypeFilter<i32>>,
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
pub struct InvoiceInvoicesFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InvoiceInvoicesFK> for OrmDataloader {
    type Value = crate::orm::invoices::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InvoiceInvoicesFK],
    ) -> Result<std::collections::HashMap<InvoiceInvoicesFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::invoices::Column::InvoiceId.as_column_ref(),
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
        Ok(crate::orm::invoices::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = InvoiceInvoicesFK(model.invoice_id.clone());
                (key, model)
            })
            .collect())
    }
}
