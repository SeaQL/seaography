use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "inventory")]
#[graphql(complex)]
#[graphql(name = "Inventory")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub inventory_id: i32,
    pub film_id: i32,
    pub store_id: i32,
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
        belongs_to = "super::film::Entity",
        from = "Column::FilmId",
        to = "super::film::Column::FilmId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Film,
    #[sea_orm(
        belongs_to = "super::store::Entity",
        from = "Column::StoreId",
        to = "super::store::Column::StoreId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Store,
    #[sea_orm(has_many = "super::rental::Entity")]
    Rental,
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl Related<super::store::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Store.def()
    }
}

impl Related<super::rental::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Rental.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilmFK(pub seaography::RelationKeyStruct<super::film::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmFK> for crate::OrmDataloader {
    type Value = super::film::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmFK],
    ) -> Result<std::collections::HashMap<FilmFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<FilmFK, Self::Value> =
            seaography::fetch_relation_data::<super::film::Entity>(
                keys,
                Relation::Film.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (FilmFK(key), model))
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
    pub async fn film<'a>(&self, ctx: &async_graphql::Context<'a>) -> Option<super::film::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Film
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = FilmFK(seaography::RelationKeyStruct(
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
