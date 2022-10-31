use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "customer")]
#[graphql(complex)]
#[graphql(name = "Customer")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub customer_id: i32,
    pub store_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub address_id: i32,
    pub active: i16,
    pub create_date: DateTimeUtc,
    pub last_update: DateTimeUtc,
}

#[derive(
    Copy,
    Clone,
    Debug,
    EnumIter,
    DeriveRelation,
    // seaography::macros::RelationsCompact
)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::address::Entity",
        from = "Column::AddressId",
        to = "super::address::Column::AddressId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Address,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Store,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
}

impl Related<super::address::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Address.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AddressFK(pub seaography::RelationKeyStruct<super::address::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<AddressFK> for crate::OrmDataloader {
    type Value = super::address::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[AddressFK],
    ) -> Result<std::collections::HashMap<AddressFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<AddressFK, Self::Value> =
            seaography::fetch_relation_data::<super::address::Entity>(
                keys,
                Relation::Address.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (AddressFK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StoreFK(pub seaography::RelationKeyStruct<super::store::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StoreFK> for crate::OrmDataloader {
    type Value = super::store::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StoreFK],
    ) -> Result<std::collections::HashMap<StoreFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<StoreFK, Self::Value> =
            seaography::fetch_relation_data::<super::store::Entity>(
                keys,
                Relation::Store.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (StoreFK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PaymentFK(pub seaography::RelationKeyStruct<super::payment::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<PaymentFK> for crate::OrmDataloader {
    type Value = Vec<super::payment::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[PaymentFK],
    ) -> Result<std::collections::HashMap<PaymentFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<PaymentFK, Self::Value> =
            seaography::fetch_relation_data::<super::payment::Entity>(
                keys,
                Relation::Payment.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (PaymentFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RentalFK(pub seaography::RelationKeyStruct<super::rental::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<RentalFK> for crate::OrmDataloader {
    type Value = Vec<super::rental::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[RentalFK],
    ) -> Result<std::collections::HashMap<RentalFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<RentalFK, Self::Value> =
            seaography::fetch_relation_data::<super::rental::Entity>(
                keys,
                Relation::Rental.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (RentalFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[async_graphql::ComplexObject]
impl Model {
    pub async fn address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<super::address::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Address
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = AddressFK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn store<'a>(&self, ctx: &async_graphql::Context<'a>) -> Option<super::store::Model> {
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
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn payment<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::payment::Filter>,
        order_by: Option<super::payment::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::payment::Model,
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
            Relation::Payment
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = PaymentFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::payment::Entity>(
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
    pub async fn rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::rental::Filter>,
        order_by: Option<super::rental::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::rental::Model,
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
            Relation::Rental
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = RentalFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::rental::Entity>(
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
