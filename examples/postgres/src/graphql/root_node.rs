use super::entities;
use sea_orm::prelude::*;
#[derive(async_graphql :: InputObject, Debug)]
pub struct PaginationInput {
    pub limit: usize,
    pub page: usize,
}
#[derive(async_graphql :: SimpleObject, Debug)]
#[graphql(concrete(name = "PaginatedCategoryResult", params(entities::category::Model)))]
#[graphql(concrete(name = "PaginatedPaymentResult", params(entities::payment::Model)))]
#[graphql(concrete(name = "PaginatedStoreResult", params(entities::store::Model)))]
#[graphql(concrete(name = "PaginatedInventoryResult", params(entities::inventory::Model)))]
#[graphql(concrete(name = "PaginatedCountryResult", params(entities::country::Model)))]
#[graphql(concrete(
    name = "PaginatedFilmCategoryResult",
    params(entities::film_category::Model)
))]
#[graphql(concrete(name = "PaginatedLanguageResult", params(entities::language::Model)))]
#[graphql(concrete(name = "PaginatedRentalResult", params(entities::rental::Model)))]
#[graphql(concrete(name = "PaginatedActorResult", params(entities::actor::Model)))]
#[graphql(concrete(name = "PaginatedStaffResult", params(entities::staff::Model)))]
#[graphql(concrete(name = "PaginatedCityResult", params(entities::city::Model)))]
#[graphql(concrete(name = "PaginatedCustomerResult", params(entities::customer::Model)))]
#[graphql(concrete(name = "PaginatedFilmResult", params(entities::film::Model)))]
#[graphql(concrete(name = "PaginatedFilmActorResult", params(entities::film_actor::Model)))]
#[graphql(concrete(name = "PaginatedAddressResult", params(entities::address::Model)))]
pub struct PaginatedResult<T: async_graphql::ObjectType> {
    pub data: Vec<T>,
    pub pages: usize,
    pub current: usize,
}
pub struct QueryRoot;
#[async_graphql::Object]
impl QueryRoot {
    async fn category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::category::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::category::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::category::Entity::find()
            .filter(entities::category::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::category::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::category::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn payment<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::payment::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::payment::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::payment::Entity::find().filter(entities::payment::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::payment::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::payment::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn store<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::store::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::store::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::store::Entity::find().filter(entities::store::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::store::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::store::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn inventory<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::inventory::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::inventory::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::inventory::Entity::find()
            .filter(entities::inventory::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::inventory::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::inventory::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn country<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::country::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::country::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::country::Entity::find().filter(entities::country::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::country::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::country::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn film_category<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::film_category::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::film_category::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::film_category::Entity::find()
            .filter(entities::film_category::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::film_category::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::film_category::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn language<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::language::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::language::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::language::Entity::find()
            .filter(entities::language::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::language::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::language::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn rental<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::rental::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::rental::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::rental::Entity::find().filter(entities::rental::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::rental::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::rental::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn actor<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::actor::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::actor::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::actor::Entity::find().filter(entities::actor::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::actor::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::actor::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn staff<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::staff::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::staff::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::staff::Entity::find().filter(entities::staff::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::staff::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::staff::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn city<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::city::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::city::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::city::Entity::find().filter(entities::city::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::city::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::city::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn customer<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::customer::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::customer::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::customer::Entity::find()
            .filter(entities::customer::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::customer::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::customer::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn film<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::film::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::film::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::film::Entity::find().filter(entities::film::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::film::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::film::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn film_actor<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::film_actor::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::film_actor::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::film_actor::Entity::find()
            .filter(entities::film_actor::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::film_actor::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::film_actor::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn address<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::address::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::address::Model> {
        println!("filters: {:?}", filters);
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt =
            entities::address::Entity::find().filter(entities::address::filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::address::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::address::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
}
