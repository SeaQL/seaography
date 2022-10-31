use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "rental")]
#[graphql(complex)]
#[graphql(name = "Rental")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub rental_id: i32,
    pub rental_date: DateTimeUtc,
    pub inventory_id: i32,
    pub customer_id: i32,
    pub return_date: Option<DateTimeUtc>,
    pub staff_id: i16,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation,
    // seaography::macros::RelationsCompact
)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::customer::Entity",
        from = "Column::CustomerId",
        to = "super::customer::Column::CustomerId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Customer,
    #[sea_orm(
        belongs_to = "super::inventory::Entity",
        from = "Column::InventoryId",
        to = "super::inventory::Column::InventoryId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Inventory,
    #[sea_orm(
        belongs_to = "super::staff::Entity",
        from = "Column::StaffId",
        to = "super::staff::Column::StaffId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Staff,
    #[sea_orm(has_many = "super::payment::Entity")]
    Payment,
}

impl Related<super::customer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl Related<super::staff::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Staff.def()
    }
}

impl Related<super::payment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Payment.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomerFK(pub seaography::RelationKeyStruct<super::customer::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<CustomerFK> for crate::OrmDataloader {
    type Value = super::customer::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[CustomerFK],
    ) -> Result<std::collections::HashMap<CustomerFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<CustomerFK, Self::Value> =
            seaography::fetch_relation_data::<super::customer::Entity>(
                keys,
                Relation::Customer.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (CustomerFK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryFK(pub seaography::RelationKeyStruct<super::inventory::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InventoryFK> for crate::OrmDataloader {
    type Value = super::inventory::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InventoryFK],
    ) -> Result<std::collections::HashMap<InventoryFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<InventoryFK, Self::Value> =
            seaography::fetch_relation_data::<super::inventory::Entity>(
                keys,
                Relation::Inventory.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (InventoryFK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StaffFK(pub seaography::RelationKeyStruct<super::staff::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<StaffFK> for crate::OrmDataloader {
    type Value = super::staff::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[StaffFK],
    ) -> Result<std::collections::HashMap<StaffFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<StaffFK, Self::Value> =
            seaography::fetch_relation_data::<super::staff::Entity>(
                keys,
                Relation::Staff.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (StaffFK(key), model))
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
#[async_graphql::ComplexObject]
impl Model {
    pub async fn customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<super::customer::Model> {
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
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<super::inventory::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Inventory
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = InventoryFK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn staff<'a>(&self, ctx: &async_graphql::Context<'a>) -> Option<super::staff::Model> {
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
}