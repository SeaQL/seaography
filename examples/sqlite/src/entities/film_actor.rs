use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
    seaography::macros::Filter,
)]
#[sea_orm(table_name = "film_actor")]
#[graphql(complex)]
#[graphql(name = "FilmActor")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub actor_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub film_id: i32,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation,
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
        belongs_to = "super::actor::Entity",
        from = "Column::ActorId",
        to = "super::actor::Column::ActorId",
        on_update = "Cascade",
        on_delete = "NoAction"
    )]
    Actor,
}

impl Related<super::film::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Film.def()
    }
}

impl Related<super::actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Actor.def()
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
pub struct ActorFK(pub seaography::RelationKeyStruct<super::actor::Entity>);
#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<ActorFK> for crate::OrmDataloader {
    type Value = super::actor::Model;
    type Error = std::sync::Arc<sea_orm::error::DbErr>;
    async fn load(
        &self,
        keys: &[ActorFK],
    ) -> Result<std::collections::HashMap<ActorFK, Self::Value>, Self::Error> {
        let keys: Vec<_> = keys.into_iter().map(|key| key.0.to_owned()).collect();
        let data: std::collections::HashMap<ActorFK, Self::Value> =
            seaography::fetch_relation_data::<super::actor::Entity>(
                keys,
                Relation::Actor.def(),
                &self.db,
            )
            .await?
            .into_iter()
            .map(|(key, model)| (ActorFK(key), model))
            .collect();
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
    pub async fn actor<'a>(&self, ctx: &async_graphql::Context<'a>) -> Option<super::actor::Model> {
        use ::std::str::FromStr;
        use seaography::heck::ToSnakeCase;
        let data_loader = ctx
            .data::<async_graphql::dataloader::DataLoader<crate::OrmDataloader>>()
            .unwrap();
        let from_column: Column = Column::from_str(
            Relation::Actor
                .def()
                .from_col
                .to_string()
                .to_snake_case()
                .as_str(),
        )
        .unwrap();
        let key = ActorFK(seaography::RelationKeyStruct(
            self.get(from_column),
            None,
            None,
        ));
        let data: Option<_> = data_loader.load_one(key).await.unwrap();
        data
    }
}