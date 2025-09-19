use chrono::{Utc};
use sea_orm::{
    DatabaseConnection,
    entity::{ActiveModelTrait, EntityTrait, IntoActiveModel, ModelTrait},
    error::DbErr,
};
use uuid::Uuid;

use crate::{
    entities::{Account, Drawing, Object, Project},
    types::{Fill, Shape, Stroke},
};

#[derive(Clone)]
pub struct Backend {
    pub db: DatabaseConnection,
}

impl Backend {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl Backend {
    pub async fn create_account(
        &self,
        name: impl Into<String>,
        email: impl Into<String>,
    ) -> Result<Account, DbErr> {
        let model = Account {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name: name.into(),
            email: email.into(),
        };

        self.insert(model.clone()).await?;
        println!("Created account {}", model.id);
        Ok(model)
    }

    pub async fn create_project(&self, name: impl Into<String>) -> Result<Project, DbErr> {
        let model = Project {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name: name.into(),
        };

        self.insert(model.clone()).await?;
        println!("Created project {}", model.id);
        Ok(model)
    }

    pub async fn create_drawing(
        &self,
        project_id: Uuid,
        name: impl Into<String>,
        width: i64,
        height: i64,
    ) -> Result<Drawing, DbErr> {
        let name = name.into();
        let model = Drawing {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            project_id,
            name,
            width,
            height,
            // tags: vec!["one".to_string(), "two".to_string()],
            // data: json!(["one", "two"]),
        };

        self.insert(model.clone()).await?;
        println!("Created drawing {}", model.id);
        Ok(model)
    }

    pub async fn create_object(
        &self,
        project_id: Uuid,
        drawing_id: Uuid,
        fill: Fill,
        stroke: Stroke,
        shape: Shape,
    ) -> Result<Object, DbErr> {
        let model = Object {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            project_id,
            drawing_id,
            fill,
            stroke,
            shape,
        };

        self.insert(model.clone()).await?;
        println!("Created object {}", model.id);
        Ok(model)
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
