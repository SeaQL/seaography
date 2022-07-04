use super::entities;
use sea_orm::prelude::*;
#[derive(async_graphql :: InputObject, Debug)]
pub struct PaginationInput {
    pub limit: usize,
    pub page: usize,
}
#[derive(async_graphql :: SimpleObject, Debug)]
#[graphql(concrete(name = "PaginatedTracksResult", params(entities::tracks::Model)))]
#[graphql(concrete(name = "PaginatedArtistsResult", params(entities::artists::Model)))]
#[graphql(concrete(name = "PaginatedEmployeesResult", params(entities::employees::Model)))]
#[graphql(concrete(
    name = "PaginatedInvoiceItemsResult",
    params(entities::invoice_items::Model)
))]
#[graphql(concrete(name = "PaginatedCustomersResult", params(entities::customers::Model)))]
#[graphql(concrete(name = "PaginatedPlaylistsResult", params(entities::playlists::Model)))]
#[graphql(concrete(name = "PaginatedInvoicesResult", params(entities::invoices::Model)))]
#[graphql(concrete(
    name = "PaginatedMediaTypesResult",
    params(entities::media_types::Model)
))]
#[graphql(concrete(name = "PaginatedGenresResult", params(entities::genres::Model)))]
#[graphql(concrete(name = "PaginatedAlbumsResult", params(entities::albums::Model)))]
#[graphql(concrete(
    name = "PaginatedPlaylistTrackResult",
    params(entities::playlist_track::Model)
))]
pub struct PaginatedResult<T: async_graphql::ObjectType> {
    pub data: Vec<T>,
    pub pages: usize,
    pub current: usize,
}
pub struct QueryRoot;
#[async_graphql::Object]
impl QueryRoot {
    async fn tracks<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::tracks::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::tracks::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(root_filter: Option<entities::tracks::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(track_id) = current_filter.track_id {
                    if let Some(eq_value) = track_id.eq {
                        condition = condition.add(entities::tracks::Column::TrackId.eq(eq_value))
                    }
                    if let Some(ne_value) = track_id.ne {
                        condition = condition.add(entities::tracks::Column::TrackId.ne(ne_value))
                    }
                }
                if let Some(name) = current_filter.name {
                    if let Some(eq_value) = name.eq {
                        condition = condition.add(entities::tracks::Column::Name.eq(eq_value))
                    }
                    if let Some(ne_value) = name.ne {
                        condition = condition.add(entities::tracks::Column::Name.ne(ne_value))
                    }
                }
                if let Some(album_id) = current_filter.album_id {
                    if let Some(eq_value) = album_id.eq {
                        condition = condition.add(entities::tracks::Column::AlbumId.eq(eq_value))
                    }
                    if let Some(ne_value) = album_id.ne {
                        condition = condition.add(entities::tracks::Column::AlbumId.ne(ne_value))
                    }
                }
                if let Some(media_type_id) = current_filter.media_type_id {
                    if let Some(eq_value) = media_type_id.eq {
                        condition =
                            condition.add(entities::tracks::Column::MediaTypeId.eq(eq_value))
                    }
                    if let Some(ne_value) = media_type_id.ne {
                        condition =
                            condition.add(entities::tracks::Column::MediaTypeId.ne(ne_value))
                    }
                }
                if let Some(genre_id) = current_filter.genre_id {
                    if let Some(eq_value) = genre_id.eq {
                        condition = condition.add(entities::tracks::Column::GenreId.eq(eq_value))
                    }
                    if let Some(ne_value) = genre_id.ne {
                        condition = condition.add(entities::tracks::Column::GenreId.ne(ne_value))
                    }
                }
                if let Some(composer) = current_filter.composer {
                    if let Some(eq_value) = composer.eq {
                        condition = condition.add(entities::tracks::Column::Composer.eq(eq_value))
                    }
                    if let Some(ne_value) = composer.ne {
                        condition = condition.add(entities::tracks::Column::Composer.ne(ne_value))
                    }
                }
                if let Some(milliseconds) = current_filter.milliseconds {
                    if let Some(eq_value) = milliseconds.eq {
                        condition =
                            condition.add(entities::tracks::Column::Milliseconds.eq(eq_value))
                    }
                    if let Some(ne_value) = milliseconds.ne {
                        condition =
                            condition.add(entities::tracks::Column::Milliseconds.ne(ne_value))
                    }
                }
                if let Some(bytes) = current_filter.bytes {
                    if let Some(eq_value) = bytes.eq {
                        condition = condition.add(entities::tracks::Column::Bytes.eq(eq_value))
                    }
                    if let Some(ne_value) = bytes.ne {
                        condition = condition.add(entities::tracks::Column::Bytes.ne(ne_value))
                    }
                }
                if let Some(unit_price) = current_filter.unit_price {
                    if let Some(eq_value) = unit_price.eq {
                        condition = condition.add(entities::tracks::Column::UnitPrice.eq(eq_value))
                    }
                    if let Some(ne_value) = unit_price.ne {
                        condition = condition.add(entities::tracks::Column::UnitPrice.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::tracks::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::tracks::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::tracks::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn artists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::artists::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::artists::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(root_filter: Option<entities::artists::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(artist_id) = current_filter.artist_id {
                    if let Some(eq_value) = artist_id.eq {
                        condition = condition.add(entities::artists::Column::ArtistId.eq(eq_value))
                    }
                    if let Some(ne_value) = artist_id.ne {
                        condition = condition.add(entities::artists::Column::ArtistId.ne(ne_value))
                    }
                }
                if let Some(name) = current_filter.name {
                    if let Some(eq_value) = name.eq {
                        condition = condition.add(entities::artists::Column::Name.eq(eq_value))
                    }
                    if let Some(ne_value) = name.ne {
                        condition = condition.add(entities::artists::Column::Name.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::artists::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::artists::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::artists::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn employees<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::employees::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::employees::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::employees::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(employee_id) = current_filter.employee_id {
                    if let Some(eq_value) = employee_id.eq {
                        condition =
                            condition.add(entities::employees::Column::EmployeeId.eq(eq_value))
                    }
                    if let Some(ne_value) = employee_id.ne {
                        condition =
                            condition.add(entities::employees::Column::EmployeeId.ne(ne_value))
                    }
                }
                if let Some(last_name) = current_filter.last_name {
                    if let Some(eq_value) = last_name.eq {
                        condition =
                            condition.add(entities::employees::Column::LastName.eq(eq_value))
                    }
                    if let Some(ne_value) = last_name.ne {
                        condition =
                            condition.add(entities::employees::Column::LastName.ne(ne_value))
                    }
                }
                if let Some(first_name) = current_filter.first_name {
                    if let Some(eq_value) = first_name.eq {
                        condition =
                            condition.add(entities::employees::Column::FirstName.eq(eq_value))
                    }
                    if let Some(ne_value) = first_name.ne {
                        condition =
                            condition.add(entities::employees::Column::FirstName.ne(ne_value))
                    }
                }
                if let Some(title) = current_filter.title {
                    if let Some(eq_value) = title.eq {
                        condition = condition.add(entities::employees::Column::Title.eq(eq_value))
                    }
                    if let Some(ne_value) = title.ne {
                        condition = condition.add(entities::employees::Column::Title.ne(ne_value))
                    }
                }
                if let Some(reports_to) = current_filter.reports_to {
                    if let Some(eq_value) = reports_to.eq {
                        condition =
                            condition.add(entities::employees::Column::ReportsTo.eq(eq_value))
                    }
                    if let Some(ne_value) = reports_to.ne {
                        condition =
                            condition.add(entities::employees::Column::ReportsTo.ne(ne_value))
                    }
                }
                if let Some(birth_date) = current_filter.birth_date {
                    if let Some(eq_value) = birth_date.eq {
                        condition =
                            condition.add(entities::employees::Column::BirthDate.eq(eq_value))
                    }
                    if let Some(ne_value) = birth_date.ne {
                        condition =
                            condition.add(entities::employees::Column::BirthDate.ne(ne_value))
                    }
                }
                if let Some(hire_date) = current_filter.hire_date {
                    if let Some(eq_value) = hire_date.eq {
                        condition =
                            condition.add(entities::employees::Column::HireDate.eq(eq_value))
                    }
                    if let Some(ne_value) = hire_date.ne {
                        condition =
                            condition.add(entities::employees::Column::HireDate.ne(ne_value))
                    }
                }
                if let Some(address) = current_filter.address {
                    if let Some(eq_value) = address.eq {
                        condition = condition.add(entities::employees::Column::Address.eq(eq_value))
                    }
                    if let Some(ne_value) = address.ne {
                        condition = condition.add(entities::employees::Column::Address.ne(ne_value))
                    }
                }
                if let Some(city) = current_filter.city {
                    if let Some(eq_value) = city.eq {
                        condition = condition.add(entities::employees::Column::City.eq(eq_value))
                    }
                    if let Some(ne_value) = city.ne {
                        condition = condition.add(entities::employees::Column::City.ne(ne_value))
                    }
                }
                if let Some(state) = current_filter.state {
                    if let Some(eq_value) = state.eq {
                        condition = condition.add(entities::employees::Column::State.eq(eq_value))
                    }
                    if let Some(ne_value) = state.ne {
                        condition = condition.add(entities::employees::Column::State.ne(ne_value))
                    }
                }
                if let Some(country) = current_filter.country {
                    if let Some(eq_value) = country.eq {
                        condition = condition.add(entities::employees::Column::Country.eq(eq_value))
                    }
                    if let Some(ne_value) = country.ne {
                        condition = condition.add(entities::employees::Column::Country.ne(ne_value))
                    }
                }
                if let Some(postal_code) = current_filter.postal_code {
                    if let Some(eq_value) = postal_code.eq {
                        condition =
                            condition.add(entities::employees::Column::PostalCode.eq(eq_value))
                    }
                    if let Some(ne_value) = postal_code.ne {
                        condition =
                            condition.add(entities::employees::Column::PostalCode.ne(ne_value))
                    }
                }
                if let Some(phone) = current_filter.phone {
                    if let Some(eq_value) = phone.eq {
                        condition = condition.add(entities::employees::Column::Phone.eq(eq_value))
                    }
                    if let Some(ne_value) = phone.ne {
                        condition = condition.add(entities::employees::Column::Phone.ne(ne_value))
                    }
                }
                if let Some(fax) = current_filter.fax {
                    if let Some(eq_value) = fax.eq {
                        condition = condition.add(entities::employees::Column::Fax.eq(eq_value))
                    }
                    if let Some(ne_value) = fax.ne {
                        condition = condition.add(entities::employees::Column::Fax.ne(ne_value))
                    }
                }
                if let Some(email) = current_filter.email {
                    if let Some(eq_value) = email.eq {
                        condition = condition.add(entities::employees::Column::Email.eq(eq_value))
                    }
                    if let Some(ne_value) = email.ne {
                        condition = condition.add(entities::employees::Column::Email.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::employees::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::employees::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::employees::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn invoice_items<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::invoice_items::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::invoice_items::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::invoice_items::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(invoice_line_id) = current_filter.invoice_line_id {
                    if let Some(eq_value) = invoice_line_id.eq {
                        condition = condition
                            .add(entities::invoice_items::Column::InvoiceLineId.eq(eq_value))
                    }
                    if let Some(ne_value) = invoice_line_id.ne {
                        condition = condition
                            .add(entities::invoice_items::Column::InvoiceLineId.ne(ne_value))
                    }
                }
                if let Some(invoice_id) = current_filter.invoice_id {
                    if let Some(eq_value) = invoice_id.eq {
                        condition =
                            condition.add(entities::invoice_items::Column::InvoiceId.eq(eq_value))
                    }
                    if let Some(ne_value) = invoice_id.ne {
                        condition =
                            condition.add(entities::invoice_items::Column::InvoiceId.ne(ne_value))
                    }
                }
                if let Some(track_id) = current_filter.track_id {
                    if let Some(eq_value) = track_id.eq {
                        condition =
                            condition.add(entities::invoice_items::Column::TrackId.eq(eq_value))
                    }
                    if let Some(ne_value) = track_id.ne {
                        condition =
                            condition.add(entities::invoice_items::Column::TrackId.ne(ne_value))
                    }
                }
                if let Some(unit_price) = current_filter.unit_price {
                    if let Some(eq_value) = unit_price.eq {
                        condition =
                            condition.add(entities::invoice_items::Column::UnitPrice.eq(eq_value))
                    }
                    if let Some(ne_value) = unit_price.ne {
                        condition =
                            condition.add(entities::invoice_items::Column::UnitPrice.ne(ne_value))
                    }
                }
                if let Some(quantity) = current_filter.quantity {
                    if let Some(eq_value) = quantity.eq {
                        condition =
                            condition.add(entities::invoice_items::Column::Quantity.eq(eq_value))
                    }
                    if let Some(ne_value) = quantity.ne {
                        condition =
                            condition.add(entities::invoice_items::Column::Quantity.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::invoice_items::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::invoice_items::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::invoice_items::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn customers<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::customers::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::customers::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::customers::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(customer_id) = current_filter.customer_id {
                    if let Some(eq_value) = customer_id.eq {
                        condition =
                            condition.add(entities::customers::Column::CustomerId.eq(eq_value))
                    }
                    if let Some(ne_value) = customer_id.ne {
                        condition =
                            condition.add(entities::customers::Column::CustomerId.ne(ne_value))
                    }
                }
                if let Some(first_name) = current_filter.first_name {
                    if let Some(eq_value) = first_name.eq {
                        condition =
                            condition.add(entities::customers::Column::FirstName.eq(eq_value))
                    }
                    if let Some(ne_value) = first_name.ne {
                        condition =
                            condition.add(entities::customers::Column::FirstName.ne(ne_value))
                    }
                }
                if let Some(last_name) = current_filter.last_name {
                    if let Some(eq_value) = last_name.eq {
                        condition =
                            condition.add(entities::customers::Column::LastName.eq(eq_value))
                    }
                    if let Some(ne_value) = last_name.ne {
                        condition =
                            condition.add(entities::customers::Column::LastName.ne(ne_value))
                    }
                }
                if let Some(company) = current_filter.company {
                    if let Some(eq_value) = company.eq {
                        condition = condition.add(entities::customers::Column::Company.eq(eq_value))
                    }
                    if let Some(ne_value) = company.ne {
                        condition = condition.add(entities::customers::Column::Company.ne(ne_value))
                    }
                }
                if let Some(address) = current_filter.address {
                    if let Some(eq_value) = address.eq {
                        condition = condition.add(entities::customers::Column::Address.eq(eq_value))
                    }
                    if let Some(ne_value) = address.ne {
                        condition = condition.add(entities::customers::Column::Address.ne(ne_value))
                    }
                }
                if let Some(city) = current_filter.city {
                    if let Some(eq_value) = city.eq {
                        condition = condition.add(entities::customers::Column::City.eq(eq_value))
                    }
                    if let Some(ne_value) = city.ne {
                        condition = condition.add(entities::customers::Column::City.ne(ne_value))
                    }
                }
                if let Some(state) = current_filter.state {
                    if let Some(eq_value) = state.eq {
                        condition = condition.add(entities::customers::Column::State.eq(eq_value))
                    }
                    if let Some(ne_value) = state.ne {
                        condition = condition.add(entities::customers::Column::State.ne(ne_value))
                    }
                }
                if let Some(country) = current_filter.country {
                    if let Some(eq_value) = country.eq {
                        condition = condition.add(entities::customers::Column::Country.eq(eq_value))
                    }
                    if let Some(ne_value) = country.ne {
                        condition = condition.add(entities::customers::Column::Country.ne(ne_value))
                    }
                }
                if let Some(postal_code) = current_filter.postal_code {
                    if let Some(eq_value) = postal_code.eq {
                        condition =
                            condition.add(entities::customers::Column::PostalCode.eq(eq_value))
                    }
                    if let Some(ne_value) = postal_code.ne {
                        condition =
                            condition.add(entities::customers::Column::PostalCode.ne(ne_value))
                    }
                }
                if let Some(phone) = current_filter.phone {
                    if let Some(eq_value) = phone.eq {
                        condition = condition.add(entities::customers::Column::Phone.eq(eq_value))
                    }
                    if let Some(ne_value) = phone.ne {
                        condition = condition.add(entities::customers::Column::Phone.ne(ne_value))
                    }
                }
                if let Some(fax) = current_filter.fax {
                    if let Some(eq_value) = fax.eq {
                        condition = condition.add(entities::customers::Column::Fax.eq(eq_value))
                    }
                    if let Some(ne_value) = fax.ne {
                        condition = condition.add(entities::customers::Column::Fax.ne(ne_value))
                    }
                }
                if let Some(email) = current_filter.email {
                    if let Some(eq_value) = email.eq {
                        condition = condition.add(entities::customers::Column::Email.eq(eq_value))
                    }
                    if let Some(ne_value) = email.ne {
                        condition = condition.add(entities::customers::Column::Email.ne(ne_value))
                    }
                }
                if let Some(support_rep_id) = current_filter.support_rep_id {
                    if let Some(eq_value) = support_rep_id.eq {
                        condition =
                            condition.add(entities::customers::Column::SupportRepId.eq(eq_value))
                    }
                    if let Some(ne_value) = support_rep_id.ne {
                        condition =
                            condition.add(entities::customers::Column::SupportRepId.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::customers::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::customers::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::customers::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn playlists<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::playlists::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::playlists::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::playlists::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(playlist_id) = current_filter.playlist_id {
                    if let Some(eq_value) = playlist_id.eq {
                        condition =
                            condition.add(entities::playlists::Column::PlaylistId.eq(eq_value))
                    }
                    if let Some(ne_value) = playlist_id.ne {
                        condition =
                            condition.add(entities::playlists::Column::PlaylistId.ne(ne_value))
                    }
                }
                if let Some(name) = current_filter.name {
                    if let Some(eq_value) = name.eq {
                        condition = condition.add(entities::playlists::Column::Name.eq(eq_value))
                    }
                    if let Some(ne_value) = name.ne {
                        condition = condition.add(entities::playlists::Column::Name.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::playlists::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::playlists::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::playlists::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn invoices<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::invoices::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::invoices::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(root_filter: Option<entities::invoices::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(invoice_id) = current_filter.invoice_id {
                    if let Some(eq_value) = invoice_id.eq {
                        condition =
                            condition.add(entities::invoices::Column::InvoiceId.eq(eq_value))
                    }
                    if let Some(ne_value) = invoice_id.ne {
                        condition =
                            condition.add(entities::invoices::Column::InvoiceId.ne(ne_value))
                    }
                }
                if let Some(customer_id) = current_filter.customer_id {
                    if let Some(eq_value) = customer_id.eq {
                        condition =
                            condition.add(entities::invoices::Column::CustomerId.eq(eq_value))
                    }
                    if let Some(ne_value) = customer_id.ne {
                        condition =
                            condition.add(entities::invoices::Column::CustomerId.ne(ne_value))
                    }
                }
                if let Some(invoice_date) = current_filter.invoice_date {
                    if let Some(eq_value) = invoice_date.eq {
                        condition =
                            condition.add(entities::invoices::Column::InvoiceDate.eq(eq_value))
                    }
                    if let Some(ne_value) = invoice_date.ne {
                        condition =
                            condition.add(entities::invoices::Column::InvoiceDate.ne(ne_value))
                    }
                }
                if let Some(billing_address) = current_filter.billing_address {
                    if let Some(eq_value) = billing_address.eq {
                        condition =
                            condition.add(entities::invoices::Column::BillingAddress.eq(eq_value))
                    }
                    if let Some(ne_value) = billing_address.ne {
                        condition =
                            condition.add(entities::invoices::Column::BillingAddress.ne(ne_value))
                    }
                }
                if let Some(billing_city) = current_filter.billing_city {
                    if let Some(eq_value) = billing_city.eq {
                        condition =
                            condition.add(entities::invoices::Column::BillingCity.eq(eq_value))
                    }
                    if let Some(ne_value) = billing_city.ne {
                        condition =
                            condition.add(entities::invoices::Column::BillingCity.ne(ne_value))
                    }
                }
                if let Some(billing_state) = current_filter.billing_state {
                    if let Some(eq_value) = billing_state.eq {
                        condition =
                            condition.add(entities::invoices::Column::BillingState.eq(eq_value))
                    }
                    if let Some(ne_value) = billing_state.ne {
                        condition =
                            condition.add(entities::invoices::Column::BillingState.ne(ne_value))
                    }
                }
                if let Some(billing_country) = current_filter.billing_country {
                    if let Some(eq_value) = billing_country.eq {
                        condition =
                            condition.add(entities::invoices::Column::BillingCountry.eq(eq_value))
                    }
                    if let Some(ne_value) = billing_country.ne {
                        condition =
                            condition.add(entities::invoices::Column::BillingCountry.ne(ne_value))
                    }
                }
                if let Some(billing_postal_code) = current_filter.billing_postal_code {
                    if let Some(eq_value) = billing_postal_code.eq {
                        condition = condition
                            .add(entities::invoices::Column::BillingPostalCode.eq(eq_value))
                    }
                    if let Some(ne_value) = billing_postal_code.ne {
                        condition = condition
                            .add(entities::invoices::Column::BillingPostalCode.ne(ne_value))
                    }
                }
                if let Some(total) = current_filter.total {
                    if let Some(eq_value) = total.eq {
                        condition = condition.add(entities::invoices::Column::Total.eq(eq_value))
                    }
                    if let Some(ne_value) = total.ne {
                        condition = condition.add(entities::invoices::Column::Total.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::invoices::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::invoices::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::invoices::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn media_types<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::media_types::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::media_types::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::media_types::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(media_type_id) = current_filter.media_type_id {
                    if let Some(eq_value) = media_type_id.eq {
                        condition =
                            condition.add(entities::media_types::Column::MediaTypeId.eq(eq_value))
                    }
                    if let Some(ne_value) = media_type_id.ne {
                        condition =
                            condition.add(entities::media_types::Column::MediaTypeId.ne(ne_value))
                    }
                }
                if let Some(name) = current_filter.name {
                    if let Some(eq_value) = name.eq {
                        condition = condition.add(entities::media_types::Column::Name.eq(eq_value))
                    }
                    if let Some(ne_value) = name.ne {
                        condition = condition.add(entities::media_types::Column::Name.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::media_types::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::media_types::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::media_types::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn genres<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::genres::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::genres::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(root_filter: Option<entities::genres::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(genre_id) = current_filter.genre_id {
                    if let Some(eq_value) = genre_id.eq {
                        condition = condition.add(entities::genres::Column::GenreId.eq(eq_value))
                    }
                    if let Some(ne_value) = genre_id.ne {
                        condition = condition.add(entities::genres::Column::GenreId.ne(ne_value))
                    }
                }
                if let Some(name) = current_filter.name {
                    if let Some(eq_value) = name.eq {
                        condition = condition.add(entities::genres::Column::Name.eq(eq_value))
                    }
                    if let Some(ne_value) = name.ne {
                        condition = condition.add(entities::genres::Column::Name.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::genres::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::genres::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::genres::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn albums<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::albums::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::albums::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(root_filter: Option<entities::albums::Filter>) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(album_id) = current_filter.album_id {
                    if let Some(eq_value) = album_id.eq {
                        condition = condition.add(entities::albums::Column::AlbumId.eq(eq_value))
                    }
                    if let Some(ne_value) = album_id.ne {
                        condition = condition.add(entities::albums::Column::AlbumId.ne(ne_value))
                    }
                }
                if let Some(title) = current_filter.title {
                    if let Some(eq_value) = title.eq {
                        condition = condition.add(entities::albums::Column::Title.eq(eq_value))
                    }
                    if let Some(ne_value) = title.ne {
                        condition = condition.add(entities::albums::Column::Title.ne(ne_value))
                    }
                }
                if let Some(artist_id) = current_filter.artist_id {
                    if let Some(eq_value) = artist_id.eq {
                        condition = condition.add(entities::albums::Column::ArtistId.eq(eq_value))
                    }
                    if let Some(ne_value) = artist_id.ne {
                        condition = condition.add(entities::albums::Column::ArtistId.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::albums::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::albums::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::albums::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
    async fn playlist_track<'a>(
        &self,
        ctx: &async_graphql::Context<'a>,
        filters: Option<entities::playlist_track::Filter>,
        pagination: Option<PaginationInput>,
    ) -> PaginatedResult<entities::playlist_track::Model> {
        println!("filters: {:?}", filters);
        fn filter_recursive(
            root_filter: Option<entities::playlist_track::Filter>,
        ) -> sea_orm::Condition {
            let mut condition = sea_orm::Condition::all();
            if let Some(current_filter) = root_filter {
                if let Some(or_filters) = current_filter.or {
                    let or_condition = or_filters.into_iter().fold(
                        sea_orm::Condition::any(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(or_condition);
                }
                if let Some(and_filters) = current_filter.and {
                    let and_condition = and_filters.into_iter().fold(
                        sea_orm::Condition::all(),
                        |fold_condition, filter| {
                            fold_condition.add(filter_recursive(Some(*filter)))
                        },
                    );
                    condition = condition.add(and_condition);
                }
                if let Some(playlist_id) = current_filter.playlist_id {
                    if let Some(eq_value) = playlist_id.eq {
                        condition =
                            condition.add(entities::playlist_track::Column::PlaylistId.eq(eq_value))
                    }
                    if let Some(ne_value) = playlist_id.ne {
                        condition =
                            condition.add(entities::playlist_track::Column::PlaylistId.ne(ne_value))
                    }
                }
                if let Some(track_id) = current_filter.track_id {
                    if let Some(eq_value) = track_id.eq {
                        condition =
                            condition.add(entities::playlist_track::Column::TrackId.eq(eq_value))
                    }
                    if let Some(ne_value) = track_id.ne {
                        condition =
                            condition.add(entities::playlist_track::Column::TrackId.ne(ne_value))
                    }
                }
            }
            condition
        }
        let db: &DatabaseConnection = ctx.data::<DatabaseConnection>().unwrap();
        let stmt = entities::playlist_track::Entity::find().filter(filter_recursive(filters));
        if let Some(pagination) = pagination {
            let paginator = stmt.paginate(db, pagination.limit);
            let data: Vec<entities::playlist_track::Model> =
                paginator.fetch_page(pagination.page).await.unwrap();
            let pages = paginator.num_pages().await.unwrap();
            PaginatedResult {
                data,
                pages,
                current: pagination.page,
            }
        } else {
            let data: Vec<entities::playlist_track::Model> = stmt.all(db).await.unwrap();
            PaginatedResult {
                data,
                pages: 1,
                current: 1,
            }
        }
    }
}
