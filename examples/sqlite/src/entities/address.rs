use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "address")]
#[graphql(complex)]
#[graphql(name = "Address")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub address_id: i32,
    pub address: String,
    pub address2: Option<String>,
    pub district: String,
    pub city_id: i32,
    pub postal_code: Option<String>,
    pub phone: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation,
    // seaography::macros::RelationsCompact
)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::city::Entity",
        from = "Column::CityId",
        to = "super::city::Column::CityId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    City,
    #[sea_orm(has_many = "super::customer::Entity")]
    Customer,
    #[sea_orm(has_many = "super::staff::Entity")]
    Staff,
    #[sea_orm(has_many = "super::store::Entity")]
    Store,
}

impl Related<super::city::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::City.def()
    }
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CityFK(pub seaography::RelationKeyStruct<super::city::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CityFK> for crate::OrmDataloader {
    type Value = super::city::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CityFK],
    ) -> Result<std::collections::HashMap<CityFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<CityFK, Self::Value> =
            seaography::fetch_relation_data::<super::city::Entity>(
                keys,
                Relation::City.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (CityFK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomerFK(pub seaography::RelationKeyStruct<super::customer::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerFK> for crate::OrmDataloader {
    type Value = Vec<super::customer::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerFK],
    ) -> Result<std::collections::HashMap<CustomerFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<CustomerFK, Self::Value> =
            seaography::fetch_relation_data::<super::customer::Entity>(
                keys,
                Relation::Customer.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (CustomerFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaffFK(pub seaography::RelationKeyStruct<super::staff::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffFK> for crate::OrmDataloader {
    type Value = Vec<super::staff::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffFK],
    ) -> Result<std::collections::HashMap<StaffFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<StaffFK, Self::Value> =
            seaography::fetch_relation_data::<super::staff::Entity>(
                keys,
                Relation::Staff.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (StaffFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreFK(pub seaography::RelationKeyStruct<super::store::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreFK> for crate::OrmDataloader {
    type Value = Vec<super::store::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreFK],
    ) -> Result<std::collections::HashMap<StoreFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<StoreFK, Self::Value> =
            seaography::fetch_relation_data::<super::store::Entity>(
                keys,
                Relation::Store.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (StoreFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[async_graphql::ComplexObject]
impl Model {
    pub async fn city<'a>(&self, ctx: &async_graphql::Context<'a>) -> Option<super::city::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::City
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = CityFK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::customer::Filter>,
        order_by: Option<super::customer::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::customer::Model,
            seaography::ExtraPaginationFields,
            async_graphql::types::connection::EmptyFields,
        >,
    > {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Customer
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = CustomerFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::customer::Entity>(
                nodes,
                false,
                false,
                Some(1),
                Some(1),
            ))
        } else {
            None
        }
    }
    pub async fn staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::staff::Filter>,
        order_by: Option<super::staff::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::staff::Model,
            seaography::ExtraPaginationFields,
            async_graphql::types::connection::EmptyFields,
        >,
    > {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Staff
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = StaffFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::staff::Entity>(
                nodes,
                false,
                false,
                Some(1),
                Some(1),
            ))
        } else {
            None
        }
    }
    pub async fn store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::store::Filter>,
        order_by: Option<super::store::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::store::Model,
            seaography::ExtraPaginationFields,
            async_graphql::types::connection::EmptyFields,
        >,
    > {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Store
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = StoreFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::store::Entity>(
                nodes,
                false,
                false,
                Some(1),
                Some(1),
            ))
        } else {
            None
        }
    }
}
