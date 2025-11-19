use sea_orm::{
    DatabaseConnection, PrimaryKeyTrait,
    entity::prelude::Uuid,
    entity::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait},
    error::DbErr,
};

#[derive(Clone)]
pub struct Backend {
    pub db: DatabaseConnection,
    pub root_account_id: Uuid,
}

impl Backend {
    pub fn new(db: DatabaseConnection, root_account_id: Uuid) -> Self {
        Self {
            db,
            root_account_id,
        }
    }
}

impl Backend {
    pub async fn find_by_id<T>(
        &self,
        id: <<<T as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType,
    ) -> Result<Option<T>, DbErr>
    where
        T: ModelTrait<Entity: EntityTrait<Model = T>>,
        <<<T as ModelTrait>::Entity as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType:
            Clone,
    {
        T::Entity::find_by_id(id).one(&self.db).await
    }

    pub async fn insert<T>(&self, model: T) -> Result<(), DbErr>
    where
        T: ModelTrait<Entity: EntityTrait<Model = T>>
            + IntoActiveModel<<<T as ModelTrait>::Entity as EntityTrait>::ActiveModel>,
    {
        <T as ModelTrait>::Entity::insert(model.into_active_model().reset_all())
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
