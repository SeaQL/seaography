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
        if let Some(employee_id) = current_filter.employee_id {
            if let Some(eq_value) = employee_id.eq {
                condition = condition.add(Column::EmployeeId.eq(eq_value))
            }
            if let Some(ne_value) = employee_id.ne {
                condition = condition.add(Column::EmployeeId.ne(ne_value))
            }
        }
        if let Some(last_name) = current_filter.last_name {
            if let Some(eq_value) = last_name.eq {
                condition = condition.add(Column::LastName.eq(eq_value))
            }
            if let Some(ne_value) = last_name.ne {
                condition = condition.add(Column::LastName.ne(ne_value))
            }
        }
        if let Some(first_name) = current_filter.first_name {
            if let Some(eq_value) = first_name.eq {
                condition = condition.add(Column::FirstName.eq(eq_value))
            }
            if let Some(ne_value) = first_name.ne {
                condition = condition.add(Column::FirstName.ne(ne_value))
            }
        }
        if let Some(title) = current_filter.title {
            if let Some(eq_value) = title.eq {
                condition = condition.add(Column::Title.eq(eq_value))
            }
            if let Some(ne_value) = title.ne {
                condition = condition.add(Column::Title.ne(ne_value))
            }
        }
        if let Some(reports_to) = current_filter.reports_to {
            if let Some(eq_value) = reports_to.eq {
                condition = condition.add(Column::ReportsTo.eq(eq_value))
            }
            if let Some(ne_value) = reports_to.ne {
                condition = condition.add(Column::ReportsTo.ne(ne_value))
            }
        }
        if let Some(birth_date) = current_filter.birth_date {
            if let Some(eq_value) = birth_date.eq {
                condition = condition.add(Column::BirthDate.eq(eq_value))
            }
            if let Some(ne_value) = birth_date.ne {
                condition = condition.add(Column::BirthDate.ne(ne_value))
            }
        }
        if let Some(hire_date) = current_filter.hire_date {
            if let Some(eq_value) = hire_date.eq {
                condition = condition.add(Column::HireDate.eq(eq_value))
            }
            if let Some(ne_value) = hire_date.ne {
                condition = condition.add(Column::HireDate.ne(ne_value))
            }
        }
        if let Some(address) = current_filter.address {
            if let Some(eq_value) = address.eq {
                condition = condition.add(Column::Address.eq(eq_value))
            }
            if let Some(ne_value) = address.ne {
                condition = condition.add(Column::Address.ne(ne_value))
            }
        }
        if let Some(city) = current_filter.city {
            if let Some(eq_value) = city.eq {
                condition = condition.add(Column::City.eq(eq_value))
            }
            if let Some(ne_value) = city.ne {
                condition = condition.add(Column::City.ne(ne_value))
            }
        }
        if let Some(state) = current_filter.state {
            if let Some(eq_value) = state.eq {
                condition = condition.add(Column::State.eq(eq_value))
            }
            if let Some(ne_value) = state.ne {
                condition = condition.add(Column::State.ne(ne_value))
            }
        }
        if let Some(country) = current_filter.country {
            if let Some(eq_value) = country.eq {
                condition = condition.add(Column::Country.eq(eq_value))
            }
            if let Some(ne_value) = country.ne {
                condition = condition.add(Column::Country.ne(ne_value))
            }
        }
        if let Some(postal_code) = current_filter.postal_code {
            if let Some(eq_value) = postal_code.eq {
                condition = condition.add(Column::PostalCode.eq(eq_value))
            }
            if let Some(ne_value) = postal_code.ne {
                condition = condition.add(Column::PostalCode.ne(ne_value))
            }
        }
        if let Some(phone) = current_filter.phone {
            if let Some(eq_value) = phone.eq {
                condition = condition.add(Column::Phone.eq(eq_value))
            }
            if let Some(ne_value) = phone.ne {
                condition = condition.add(Column::Phone.ne(ne_value))
            }
        }
        if let Some(fax) = current_filter.fax {
            if let Some(eq_value) = fax.eq {
                condition = condition.add(Column::Fax.eq(eq_value))
            }
            if let Some(ne_value) = fax.ne {
                condition = condition.add(Column::Fax.ne(ne_value))
            }
        }
        if let Some(email) = current_filter.email {
            if let Some(eq_value) = email.eq {
                condition = condition.add(Column::Email.eq(eq_value))
            }
            if let Some(ne_value) = email.ne {
                condition = condition.add(Column::Email.ne(ne_value))
            }
        }
    }
    condition
}
use crate::graphql::*;
pub use crate::orm::employees::*;
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
