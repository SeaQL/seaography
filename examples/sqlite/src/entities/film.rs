use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film")]
#[graphql(complex)]
#[graphql(name = "Film")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub release_year: Option<String>,
    pub language_id: i16,
    pub original_language_id: Option<i16>,
    pub rental_duration: i16,
    #[sea_orm(column_type = "Decimal(Some((4, 2)))")]
    pub rental_rate: Decimal,
    pub length: Option<i16>,
    #[sea_orm(column_type = "Decimal(Some((5, 2)))")]
    pub replacement_cost: Decimal,
    pub rating: Option<String>,
    pub special_features: Option<String>,
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
        belongs_to = "super::language::Entity",
        from = "Column::OriginalLanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Language2,
    #[sea_orm(
        belongs_to = "super::language::Entity",
        from = "Column::LanguageId",
        to = "super::language::Column::LanguageId",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Language1,
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
    #[sea_orm(has_many = "super::inventory::Entity")]
    Inventory,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl Related<super::inventory::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Inventory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language2FK(pub seaography::RelationKeyStruct<super::language::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<Language2FK> for crate::OrmDataloader {
    type Value = super::language::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[Language2FK],
    ) -> Result<std::collections::HashMap<Language2FK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<Language2FK, Self::Value> =
            seaography::fetch_relation_data::<super::language::Entity>(
                keys,
                Relation::Language2.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (Language2FK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Language1FK(pub seaography::RelationKeyStruct<super::language::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<Language1FK> for crate::OrmDataloader {
    type Value = super::language::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[Language1FK],
    ) -> Result<std::collections::HashMap<Language1FK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<Language1FK, Self::Value> =
            seaography::fetch_relation_data::<super::language::Entity>(
                keys,
                Relation::Language1.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (Language1FK(key), model))
            .collect();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilmActorFK(pub seaography::RelationKeyStruct<super::film_actor::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmActorFK> for crate::OrmDataloader {
    type Value = Vec<super::film_actor::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmActorFK],
    ) -> Result<std::collections::HashMap<FilmActorFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<FilmActorFK, Self::Value> =
            seaography::fetch_relation_data::<super::film_actor::Entity>(
                keys,
                Relation::FilmActor.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (FilmActorFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilmCategoryFK(pub seaography::RelationKeyStruct<super::film_category::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<FilmCategoryFK> for crate::OrmDataloader {
    type Value = Vec<super::film_category::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[FilmCategoryFK],
    ) -> Result<std::collections::HashMap<FilmCategoryFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<FilmCategoryFK, Self::Value> =
            seaography::fetch_relation_data::<super::film_category::Entity>(
                keys,
                Relation::FilmCategory.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (FilmCategoryFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryFK(pub seaography::RelationKeyStruct<super::inventory::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<InventoryFK> for crate::OrmDataloader {
    type Value = Vec<super::inventory::Model>;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[InventoryFK],
    ) -> Result<std::collections::HashMap<InventoryFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        use seaography::itertools::Itertools;
        let data: std::collections::HashMap<InventoryFK, Self::Value> =
            seaography::fetch_relation_data::<super::inventory::Entity>(
                keys,
                Relation::Inventory.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (InventoryFK(key), model))
            .into_group_map();
        Ok(data)
    }
}
#[async_graphql::ComplexObject]
impl Model {
    pub async fn language2<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<super::language::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Language2
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = Language2FK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn language1<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
    ) -> Option<super::language::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Language1
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = Language1FK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
    pub async fn film_actor<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::film_actor::Filter>,
        order_by: Option<super::film_actor::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::film_actor::Model,
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
            Relation::FilmActor
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = FilmActorFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::film_actor::Entity>(
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
    pub async fn film_category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::film_category::Filter>,
        order_by: Option<super::film_category::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::film_category::Model,
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
            Relation::FilmCategory
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = FilmCategoryFK(seaography::RelationKeyStruct(
            self.get(from_column),
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(
                seaography::data_to_connection::<super::film_category::Entity>(
                    nodes,
                    false,
                    false,
                    Some(1),
                    Some(1),
                ),
            )
        } else {
            None
        }
    }
    pub async fn inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<super::inventory::Filter>,
        order_by: Option<super::inventory::OrderBy>,
    ) -> Option<
        async_graphql::types::connection::Connection<
            String,
            super::inventory::Model,
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
            filters,
            order_by,
        ));
        let option_nodes: Option<_> = data_loader.load_one(key).await.unwrap();
        if let Some(nodes) = option_nodes {
            Some(seaography::data_to_connection::<super::inventory::Entity>(
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
