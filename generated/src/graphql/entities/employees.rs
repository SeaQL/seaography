use crate::graphql::*;
pub use crate::orm::employees::*;
use sea_orm::prelude::*;
#[async_graphql::Object(name = "Employees")]
impl Model {
    pub async fn employee_id(&self) -> &i32 {
        &self.employee_id
    }
    pub async fn last_name(&self) -> &String {
        &self.last_name
    }
    pub async fn first_name(&self) -> &String {
        &self.first_name
    }
    pub async fn title(&self) -> &Option<String> {
        &self.title
    }
    pub async fn reports_to(&self) -> &Option<i32> {
        &self.reports_to
    }
    pub async fn birth_date(&self) -> &Option<DateTime> {
        &self.birth_date
    }
    pub async fn hire_date(&self) -> &Option<DateTime> {
        &self.hire_date
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
    pub async fn email(&self) -> &Option<String> {
        &self.email
    }
    pub async fn employee_employees<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::employees::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = EmployeeEmployeesFK(self.employee_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn reports_to_employees<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::employees::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = ReportsToEmployeesFK(self.reports_to.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
    pub async fn employee_customers<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Vec<crate::orm::customers::Model> {
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<OrmDataloader>>()
            .unwrap();
        let key = EmployeeCustomersFK(self.employee_id.clone());
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data.unwrap_or(vec![])
    }
}
#[derive(async_graphql :: InputObject, Debug)]
#[graphql(name = "EmployeesFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    pub employee_id: Option<TypeFilter<i32>>,
    pub last_name: Option<TypeFilter<String>>,
    pub first_name: Option<TypeFilter<String>>,
    pub title: Option<TypeFilter<String>>,
    pub reports_to: Option<TypeFilter<i32>>,
    pub birth_date: Option<TypeFilter<DateTime>>,
    pub hire_date: Option<TypeFilter<DateTime>>,
    pub address: Option<TypeFilter<String>>,
    pub city: Option<TypeFilter<String>>,
    pub state: Option<TypeFilter<String>>,
    pub country: Option<TypeFilter<String>>,
    pub postal_code: Option<TypeFilter<String>>,
    pub phone: Option<TypeFilter<String>>,
    pub fax: Option<TypeFilter<String>>,
    pub email: Option<TypeFilter<String>>,
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EmployeeEmployeesFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<EmployeeEmployeesFK> for OrmDataloader {
    type Value = Vec<crate::orm::employees::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[EmployeeEmployeesFK],
    ) -> Result<std::collections::HashMap<EmployeeEmployeesFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::employees::Column::ReportsTo.as_column_ref(),
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
        Ok(crate::orm::employees::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = EmployeeEmployeesFK(model.reports_to.unwrap().clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ReportsToEmployeesFK(Option<i32>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<ReportsToEmployeesFK> for OrmDataloader {
    type Value = Vec<crate::orm::employees::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[ReportsToEmployeesFK],
    ) -> Result<std::collections::HashMap<ReportsToEmployeesFK, Self::Value>, Self::Error> {
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
        use itertools::Itertools;
        Ok(crate::orm::employees::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = ReportsToEmployeesFK(Some(model.employee_id).clone());
                (key, model)
            })
            .into_group_map())
    }
}
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct EmployeeCustomersFK(i32);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<EmployeeCustomersFK> for OrmDataloader {
    type Value = Vec<crate::orm::customers::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[EmployeeCustomersFK],
    ) -> Result<std::collections::HashMap<EmployeeCustomersFK, Self::Value>, Self::Error> {
        let filter = sea_orm::Condition::all().add(sea_orm::sea_query::SimpleExpr::Binary(
            Box::new(sea_orm::sea_query::SimpleExpr::Tuple(vec![
                sea_orm::sea_query::Expr::col(
                    crate::orm::customers::Column::SupportRepId.as_column_ref(),
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
        Ok(crate::orm::customers::Entity::find()
            .filter(filter)
            .all(&self.db)
            .await?
            .into_iter()
            .map(|model| {
                let key = EmployeeCustomersFK(model.support_rep_id.unwrap().clone());
                (key, model)
            })
            .into_group_map())
    }
}
