use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "category")]
#[graphql(complex)]
#[graphql(name = "Category")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub category_id: i16,
    pub name: String,
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
    #[sea_orm(has_many = "super::film_category::Entity")]
    FilmCategory,
}

impl Related<super::film_category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmCategory.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

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
#[async_graphql::ComplexObject]
impl Model {
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
}
