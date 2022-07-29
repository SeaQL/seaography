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
        if let Some(invoice_id) = current_filter.invoice_id {
            if let Some(eq_value) = invoice_id.eq {
                condition = condition.add(Column::InvoiceId.eq(eq_value))
            }
            if let Some(ne_value) = invoice_id.ne {
                condition = condition.add(Column::InvoiceId.ne(ne_value))
            }
        }
        if let Some(customer_id) = current_filter.customer_id {
            if let Some(eq_value) = customer_id.eq {
                condition = condition.add(Column::CustomerId.eq(eq_value))
            }
            if let Some(ne_value) = customer_id.ne {
                condition = condition.add(Column::CustomerId.ne(ne_value))
            }
        }
        if let Some(invoice_date) = current_filter.invoice_date {
            if let Some(eq_value) = invoice_date.eq {
                condition = condition.add(Column::InvoiceDate.eq(eq_value))
            }
            if let Some(ne_value) = invoice_date.ne {
                condition = condition.add(Column::InvoiceDate.ne(ne_value))
            }
        }
        if let Some(billing_address) = current_filter.billing_address {
            if let Some(eq_value) = billing_address.eq {
                condition = condition.add(Column::BillingAddress.eq(eq_value))
            }
            if let Some(ne_value) = billing_address.ne {
                condition = condition.add(Column::BillingAddress.ne(ne_value))
            }
        }
        if let Some(billing_city) = current_filter.billing_city {
            if let Some(eq_value) = billing_city.eq {
                condition = condition.add(Column::BillingCity.eq(eq_value))
            }
            if let Some(ne_value) = billing_city.ne {
                condition = condition.add(Column::BillingCity.ne(ne_value))
            }
        }
        if let Some(billing_state) = current_filter.billing_state {
            if let Some(eq_value) = billing_state.eq {
                condition = condition.add(Column::BillingState.eq(eq_value))
            }
            if let Some(ne_value) = billing_state.ne {
                condition = condition.add(Column::BillingState.ne(ne_value))
            }
        }
        if let Some(billing_country) = current_filter.billing_country {
            if let Some(eq_value) = billing_country.eq {
                condition = condition.add(Column::BillingCountry.eq(eq_value))
            }
            if let Some(ne_value) = billing_country.ne {
                condition = condition.add(Column::BillingCountry.ne(ne_value))
            }
        }
        if let Some(billing_postal_code) = current_filter.billing_postal_code {
            if let Some(eq_value) = billing_postal_code.eq {
                condition = condition.add(Column::BillingPostalCode.eq(eq_value))
            }
            if let Some(ne_value) = billing_postal_code.ne {
                condition = condition.add(Column::BillingPostalCode.ne(ne_value))
            }
        }
        if let Some(total) = current_filter.total {
            if let Some(eq_value) = total.eq {
                condition = condition.add(Column::Total.eq(eq_value))
            }
            if let Some(ne_value) = total.ne {
                condition = condition.add(Column::Total.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::invoices::*;
#[async_graphql::Object(name = "Invoices")]
impl Model {
    pub async fn invoice_id(&self) -> &i32 {
        &self.invoice_id
    }
    pub async fn customer_id(&self) -> &i32 {
        &self.customer_id
    }
    pub async fn invoice_date(&self) -> &DateTime {
        &self.invoice_date
    }
    pub async fn billing_address(&self) -> &Option<String> {
        &self.billing_address
    }
    pub async fn billing_city(&self) -> &Option<String> {
        &self.billing_city
    }
    pub async fn billing_state(&self) -> &Option<String> {
        &self.billing_state
    }
    pub async fn billing_country(&self) -> &Option<String> {
        &self.billing_country
    }
    pub async fn billing_postal_code(&self) -> &Option<String> {
        &self.billing_postal_code
    }
    pub async fn total(&self) -> &f64 {
        &self.total
    }
    pub async fn invoice_invoice_items<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::invoice_items::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = InvoiceInvoiceItemsFK(self.invoice_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn customer_customers<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> crate::orm::customers::Model {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CustomerCustomersFK(self.customer_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap()
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "InvoicesFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub invoice_id: Option<TypeFilter<i32>>,
    pub customer_id: Option<TypeFilter<i32>>,
    pub invoice_date: Option<TypeFilter<DateTime>>,
    pub billing_address: Option<TypeFilter<String>>,
    pub billing_city: Option<TypeFilter<String>>,
    pub billing_state: Option<TypeFilter<String>>,
    pub billing_country: Option<TypeFilter<String>>,
    pub billing_postal_code: Option<TypeFilter<String>>,
    pub total: Option<TypeFilter<f64>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct InvoiceInvoiceItemsFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InvoiceInvoiceItemsFK> for OrmDataloader {
    type Value = Vec<crate::orm::invoice_items::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InvoiceInvoiceItemsFK],
    ) -> Result<std::collections::HashMap<InvoiceInvoiceItemsFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::invoice_items::Column::InvoiceId.as_column_ref(),
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
                let key = InvoiceInvoiceItemsFK(model.invoice_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomerCustomersFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerCustomersFK> for OrmDataloader {
    type Value = crate::orm::customers::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerCustomersFK],
    ) -> Result<std::collections::HashMap<CustomerCustomersFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::customers::Column::CustomerId.as_column_ref(),
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
        Ok(crate::orm::customers::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CustomerCustomersFK(model.customer_id.clone());
                (key, model)
            })
            .collect())
    }
}
