use async_graphql::dynamic::{ObjectAccessor, ResolverContext, ValueAccessor};
use sea_orm::{
    sea_query::Expr, Condition, EntityTrait, Iterable, QueryFilter, QuerySelect, QueryTrait,
    Related, RelationDef,
};
use std::marker::PhantomData;

use crate::{
    recursive_prepare_condition, BuilderContext, EntityObjectBuilder, FilterInputBuilder,
    RelationBuilder, SeaResult,
};

/// utility function used to create the query filter condition
/// for a SeaORM entity using query filter inputs of related entities
pub fn get_having_conditions<T>(
    context: &'static BuilderContext,
    ctx: &ResolverContext,
    condition: Condition,
    having: Option<ValueAccessor>,
) -> SeaResult<Condition>
where
    T: EntityTrait,
{
    if let Some(having) = having {
        let having = having.object()?;
        let related = ctx.data_unchecked::<RelatedEntityFilter<T>>();
        related.apply(context, condition, &having)
    } else {
        Ok(condition)
    }
}

pub struct RelatedEntityFilterBuilder {
    pub context: &'static BuilderContext,
}

type FnFilterCondition =
    Box<dyn Fn(&'static BuilderContext, &ObjectAccessor) -> SeaResult<Option<Expr>> + Send + Sync>;

pub struct RelatedEntityFilter<E>
where
    E: EntityTrait,
{
    fields: Vec<RelatedEntityFilterField>,
    entity: PhantomData<E>,
}

pub struct RelatedEntityFilterField {
    name: String,
    filter_input: String,
    filter_condition_fn: FnFilterCondition,
}

impl<E> RelatedEntityFilter<E>
where
    E: EntityTrait,
{
    pub fn build<T>(context: &'static BuilderContext) -> Self
    where
        T: Iterable + RelationBuilder,
    {
        Self {
            fields: T::iter()
                .map(|rel| rel.get_related_entity_filter(context))
                .collect(),
            entity: PhantomData,
        }
    }

    /// (field_name, filter_input)
    pub fn field_names(&self) -> Vec<(String, String)> {
        self.fields
            .iter()
            .map(|f| (f.name.clone(), f.filter_input.clone()))
            .collect()
    }

    fn apply(
        &self,
        context: &'static BuilderContext,
        mut condition: Condition,
        having: &ObjectAccessor,
    ) -> SeaResult<Condition> {
        for field in &self.fields {
            if let Some(filter) = having.get(&field.name) {
                let filter = filter.object()?;
                if let Some(additional) = (field.filter_condition_fn)(context, &filter)? {
                    condition = condition.add(additional);
                }
            }
        }
        Ok(condition)
    }
}

impl RelatedEntityFilterBuilder {
    pub fn get_relation_via<T, R>(&self, name: &str) -> RelatedEntityFilterField
    where
        T: EntityTrait + Related<R>,
        R: EntityTrait,
    {
        RelatedEntityFilterField::new::<R>(
            self.context,
            name.to_owned(),
            <T as Related<R>>::to(),
            <T as Related<R>>::via(),
        )
    }

    pub fn get_relation<T, R>(&self, name: &str, to: RelationDef) -> RelatedEntityFilterField
    where
        T: EntityTrait,
        R: EntityTrait,
    {
        RelatedEntityFilterField::new::<R>(self.context, name.to_owned(), to, None)
    }
}

impl RelatedEntityFilterField {
    fn new<R>(
        context: &'static BuilderContext,
        name: String,
        to: RelationDef,
        via: Option<RelationDef>,
    ) -> Self
    where
        R: EntityTrait,
    {
        Self {
            name,
            filter_input: {
                let entity_object_builder = EntityObjectBuilder { context };
                let filter_input_builder = FilterInputBuilder { context };
                let object_name: String = entity_object_builder.type_name::<R>();
                filter_input_builder.type_name(&object_name)
            },
            filter_condition_fn: Box::new(move |context, filter| -> SeaResult<Option<Expr>> {
                let mut condition = recursive_prepare_condition::<R>(context, filter)?;
                if !condition.is_empty() {
                    condition = condition.add(if let Some(via) = via.clone() {
                        via
                    } else {
                        to.clone()
                    });
                    let mut subquery = R::find()
                        .select_only()
                        .expr(Expr::cust("1"))
                        .filter(condition)
                        .into_query();
                    if via.is_some() {
                        // join the junction table
                        subquery.inner_join(to.from_tbl.clone(), to.clone());
                    }
                    Ok(Some(Expr::exists(subquery)))
                } else {
                    Ok(None)
                }
            }),
        }
    }
}
