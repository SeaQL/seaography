use sea_orm::entity::prelude::*;

#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveEntityModel,
    async_graphql::SimpleObject,
)]
#[sea_orm(table_name = "actor")]
#[graphql(complex)]
#[graphql(name = "Actor")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub last_update: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation, seaography::macros::RelationsCompact)]
pub enum Relation {
    #[sea_orm(has_many = "super::film_actor::Entity")]
    FilmActor,
}

impl Related<super::film_actor::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FilmActor.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

// Recursive expansion of seaography::macros::Filter! macro
// =========================================================

#[derive(Debug, Clone, async_graphql::InputObject)]
#[graphql(name = "ActorFilter")]
pub struct Filter {
    pub or: Option<Vec<Box<Filter>>>,
    pub and: Option<Vec<Box<Filter>>>,
    actor_id: Option<<i32 as seaography::FilterTypeTrait>::Filter>,
    first_name: Option<<String as seaography::FilterTypeTrait>::Filter>,
    last_name: Option<<String as seaography::FilterTypeTrait>::Filter>,
    last_update: Option<<DateTimeUtc as seaography::FilterTypeTrait>::Filter>,
}
impl seaography::EntityFilter for Filter {
    fn filter_condition(&self) -> sea_orm::Condition {
        let mut condition = sea_orm::Condition::all();
        if let Some(or_filters) = &self.or {
            let or_condition = or_filters
                .iter()
                .fold(sea_orm::Condition::any(), |fold_condition, filter| {
                    fold_condition.add(filter.filter_condition())
                });
            condition = condition.add(or_condition);
        }
        if let Some(and_filters) = &self.and {
            let and_condition = and_filters
                .iter()
                .fold(sea_orm::Condition::all(), |fold_condition, filter| {
                    fold_condition.add(filter.filter_condition())
                });
            condition = condition.add(and_condition);
        }
        if let Some(actor_id) = &self.actor_id {
            if let Some(eq_value) = seaography::FilterTrait::eq(actor_id) {
                condition = condition.add(Column::ActorId.eq(eq_value))
            }
            if let Some(ne_value) = seaography::FilterTrait::ne(actor_id) {
                condition = condition.add(Column::ActorId.ne(ne_value))
            }
            if let Some(gt_value) = seaography::FilterTrait::gt(actor_id) {
                condition = condition.add(Column::ActorId.gt(gt_value))
            }
            if let Some(gte_value) = seaography::FilterTrait::gte(actor_id) {
                condition = condition.add(Column::ActorId.gte(gte_value))
            }
            if let Some(lt_value) = seaography::FilterTrait::lt(actor_id) {
                condition = condition.add(Column::ActorId.lt(lt_value))
            }
            if let Some(lte_value) = seaography::FilterTrait::lte(actor_id) {
                condition = condition.add(Column::ActorId.lte(lte_value))
            }
            if let Some(is_in_value) = seaography::FilterTrait::is_in(actor_id) {
                condition = condition.add(Column::ActorId.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = seaography::FilterTrait::is_not_in(actor_id) {
                condition = condition.add(Column::ActorId.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = seaography::FilterTrait::is_null(actor_id) {
                if is_null_value {
                    condition = condition.add(Column::ActorId.is_null())
                }
            }
        }
        if let Some(first_name) = &self.first_name {
            if let Some(eq_value) = seaography::FilterTrait::eq(first_name) {
                condition = condition.add(Column::FirstName.eq(eq_value))
            }
            if let Some(ne_value) = seaography::FilterTrait::ne(first_name) {
                condition = condition.add(Column::FirstName.ne(ne_value))
            }
            if let Some(gt_value) = seaography::FilterTrait::gt(first_name) {
                condition = condition.add(Column::FirstName.gt(gt_value))
            }
            if let Some(gte_value) = seaography::FilterTrait::gte(first_name) {
                condition = condition.add(Column::FirstName.gte(gte_value))
            }
            if let Some(lt_value) = seaography::FilterTrait::lt(first_name) {
                condition = condition.add(Column::FirstName.lt(lt_value))
            }
            if let Some(lte_value) = seaography::FilterTrait::lte(first_name) {
                condition = condition.add(Column::FirstName.lte(lte_value))
            }
            if let Some(is_in_value) = seaography::FilterTrait::is_in(first_name) {
                condition = condition.add(Column::FirstName.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = seaography::FilterTrait::is_not_in(first_name) {
                condition = condition.add(Column::FirstName.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = seaography::FilterTrait::is_null(first_name) {
                if is_null_value {
                    condition = condition.add(Column::FirstName.is_null())
                }
            }
        }
        if let Some(last_name) = &self.last_name {
            if let Some(eq_value) = seaography::FilterTrait::eq(last_name) {
                condition = condition.add(Column::LastName.eq(eq_value))
            }
            if let Some(ne_value) = seaography::FilterTrait::ne(last_name) {
                condition = condition.add(Column::LastName.ne(ne_value))
            }
            if let Some(gt_value) = seaography::FilterTrait::gt(last_name) {
                condition = condition.add(Column::LastName.gt(gt_value))
            }
            if let Some(gte_value) = seaography::FilterTrait::gte(last_name) {
                condition = condition.add(Column::LastName.gte(gte_value))
            }
            if let Some(lt_value) = seaography::FilterTrait::lt(last_name) {
                condition = condition.add(Column::LastName.lt(lt_value))
            }
            if let Some(lte_value) = seaography::FilterTrait::lte(last_name) {
                condition = condition.add(Column::LastName.lte(lte_value))
            }
            if let Some(is_in_value) = seaography::FilterTrait::is_in(last_name) {
                condition = condition.add(Column::LastName.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = seaography::FilterTrait::is_not_in(last_name) {
                condition = condition.add(Column::LastName.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = seaography::FilterTrait::is_null(last_name) {
                if is_null_value {
                    condition = condition.add(Column::LastName.is_null())
                }
            }
        }
        if let Some(last_update) = &self.last_update {
            if let Some(eq_value) = seaography::FilterTrait::eq(last_update) {
                condition = condition.add(Column::LastUpdate.eq(eq_value))
            }
            if let Some(ne_value) = seaography::FilterTrait::ne(last_update) {
                condition = condition.add(Column::LastUpdate.ne(ne_value))
            }
            if let Some(gt_value) = seaography::FilterTrait::gt(last_update) {
                condition = condition.add(Column::LastUpdate.gt(gt_value))
            }
            if let Some(gte_value) = seaography::FilterTrait::gte(last_update) {
                condition = condition.add(Column::LastUpdate.gte(gte_value))
            }
            if let Some(lt_value) = seaography::FilterTrait::lt(last_update) {
                condition = condition.add(Column::LastUpdate.lt(lt_value))
            }
            if let Some(lte_value) = seaography::FilterTrait::lte(last_update) {
                condition = condition.add(Column::LastUpdate.lte(lte_value))
            }
            if let Some(is_in_value) = seaography::FilterTrait::is_in(last_update) {
                condition = condition.add(Column::LastUpdate.is_in(is_in_value))
            }
            if let Some(is_not_in_value) = seaography::FilterTrait::is_not_in(last_update) {
                condition = condition.add(Column::LastUpdate.is_not_in(is_not_in_value))
            }
            if let Some(is_null_value) = seaography::FilterTrait::is_null(last_update) {
                if is_null_value {
                    condition = condition.add(Column::LastUpdate.is_null())
                }
            }
        }
        condition
    }
}
#[derive(Debug, Clone, async_graphql::InputObject)]
#[graphql(name = "ActorOrderBy")]
pub struct OrderBy {
    actor_id: Option<seaography::OrderByEnum>,
    first_name: Option<seaography::OrderByEnum>,
    last_name: Option<seaography::OrderByEnum>,
    last_update: Option<seaography::OrderByEnum>,
}
impl seaography::EntityOrderBy<Entity> for OrderBy {
    fn order_by(&self, stmt: sea_orm::Select<Entity>) -> sea_orm::Select<Entity> {
        use sea_orm::QueryOrder;
        let stmt = if let Some(order_by) = self.actor_id {
            match order_by {
                seaography::OrderByEnum::Asc => {
                    stmt.order_by(Column::ActorId, sea_orm::query::Order::Asc)
                }
                seaography::OrderByEnum::Desc => {
                    stmt.order_by(Column::ActorId, sea_orm::query::Order::Desc)
                }
            }
        } else {
            stmt
        };
        let stmt = if let Some(order_by) = self.first_name {
            match order_by {
                seaography::OrderByEnum::Asc => {
                    stmt.order_by(Column::FirstName, sea_orm::query::Order::Asc)
                }
                seaography::OrderByEnum::Desc => {
                    stmt.order_by(Column::FirstName, sea_orm::query::Order::Desc)
                }
            }
        } else {
            stmt
        };
        let stmt = if let Some(order_by) = self.last_name {
            match order_by {
                seaography::OrderByEnum::Asc => {
                    stmt.order_by(Column::LastName, sea_orm::query::Order::Asc)
                }
                seaography::OrderByEnum::Desc => {
                    stmt.order_by(Column::LastName, sea_orm::query::Order::Desc)
                }
            }
        } else {
            stmt
        };
        let stmt = if let Some(order_by) = self.last_update {
            match order_by {
                seaography::OrderByEnum::Asc => {
                    stmt.order_by(Column::LastUpdate, sea_orm::query::Order::Asc)
                }
                seaography::OrderByEnum::Desc => {
                    stmt.order_by(Column::LastUpdate, sea_orm::query::Order::Desc)
                }
            }
        } else {
            stmt
        };
        stmt
    }
}
impl seaography::EnhancedEntity for Entity {
    type Entity = Entity;
    type Filter = Filter;
    type OrderBy = OrderBy;
}