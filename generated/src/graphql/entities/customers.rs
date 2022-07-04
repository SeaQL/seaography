use crate::graphql::*;
pub use crate::orm::customers::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Customers")]
impl Model {
    pub async fn customer_id(&self) -> &i32 {
        &self.customer_id
    }
    pub async fn first_name(&self) -> &String {
        &self.first_name
    }
    pub async fn last_name(&self) -> &String {
        &self.last_name
    }
    pub async fn company(&self) -> &Option<String> {
        &self.company
    }
    pub async fn address(&self) -> &Option<String> {
        &self.address
    }
    pub async fn city(&self) -> &Option<String> {
        &self.city
    }
    pub async fn state(&self) -> &Option<String> {
        &self.state
    }
    pub async fn country(&self) -> &Option<String> {
        &self.country
    }
    pub async fn postal_code(&self) -> &Option<String> {
        &self.postal_code
    }
    pub async fn phone(&self) -> &Option<String> {
        &self.phone
    }
    pub async fn fax(&self) -> &Option<String> {
        &self.fax
    }
    pub async fn email(&self) -> &String {
        &self.email
    }
    pub async fn support_rep_id(&self) -> &Option<i32> {
        &self.support_rep_id
    }
    pub async fn support_rep_employees<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<crate::orm::employees::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = SupportRepEmployeesFK(self.support_rep_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn customer_invoices<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::invoices::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = CustomerInvoicesFK(self.customer_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "CustomersFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub customer_id: Option<TypeFilter<i32>>,
    pub first_name: Option<TypeFilter<String>>,
    pub last_name: Option<TypeFilter<String>>,
    pub company: Option<TypeFilter<String>>,
    pub address: Option<TypeFilter<String>>,
    pub city: Option<TypeFilter<String>>,
    pub state: Option<TypeFilter<String>>,
    pub country: Option<TypeFilter<String>>,
    pub postal_code: Option<TypeFilter<String>>,
    pub phone: Option<TypeFilter<String>>,
    pub fax: Option<TypeFilter<String>>,
    pub email: Option<TypeFilter<String>>,
    pub support_rep_id: Option<TypeFilter<i32>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SupportRepEmployeesFK(Option<i32>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<SupportRepEmployeesFK> for OrmDataloader {
    type Value = crate::orm::employees::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[SupportRepEmployeesFK],
    ) -> Result<std::collections::HashMap<SupportRepEmployeesFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::employees::Column::EmployeeId.as_column_ref(),
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
        Ok(crate::orm::employees::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = SupportRepEmployeesFK(Some(model.employee_id).clone());
                (key, model)
            })
            .collect())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct CustomerInvoicesFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerInvoicesFK> for OrmDataloader {
    type Value = Vec<crate::orm::invoices::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerInvoicesFK],
    ) -> Result<std::collections::HashMap<CustomerInvoicesFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::invoices::Column::CustomerId.as_column_ref(),
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
        Ok(crate::orm::invoices::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = CustomerInvoicesFK(model.customer_id.clone());
                (key, model)
            })
            .into_group_map())
    }
}
