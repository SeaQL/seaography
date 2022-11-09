pub type BinaryVector = Vec<u8>;

#[derive(Debug, Clone, async_graphql::InputObject)]
#[graphql(concrete(name = "TinyIntegerFilter", params(i8)))]
#[graphql(concrete(name = "SmallIntegerFilter", params(i16)))]
#[graphql(concrete(name = "IntegerFilter", params(i32)))]
#[graphql(concrete(name = "BigIntegerFilter", params(i64)))]
#[graphql(concrete(name = "TinyUnsignedFilter", params(u8)))]
#[graphql(concrete(name = "SmallUnsignedFilter", params(u16)))]
#[graphql(concrete(name = "UnsignedFilter", params(u32)))]
#[graphql(concrete(name = "BigUnsignedFilter", params(u64)))]
#[graphql(concrete(name = "FloatFilter", params(f32)))]
#[graphql(concrete(name = "DoubleFilter", params(f64)))]
#[cfg_attr(
    feature = "with-json",
    graphql(concrete(name = "JsonFilter", params(sea_orm::prelude::Json)))
)]
// TODO #[graphql(concrete(name = "DateFilter", params()))]
// TODO #[graphql(concrete(name = "TimeFilter", params()))]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateFilter", params(sea_orm::prelude::Date)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeFilter", params(sea_orm::prelude::DateTime)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(name = "DateTimeUtcFilter", params(sea_orm::prelude::DateTimeUtc)))
)]
#[cfg_attr(
    feature = "with-chrono",
    graphql(concrete(
        name = "DateTimeWithTimeZoneFilter",
        params(sea_orm::prelude::DateTimeWithTimeZone)
    ))
)]
// TODO #[graphql(concrete(name = "TimestampFilter", params()))]
// TODO #[graphql(concrete(name = "TimestampWithTimeZoneFilter", params()))]
#[cfg_attr(
    feature = "with-decimal",
    graphql(concrete(name = "DecimalFilter", params(sea_orm::prelude::Decimal)))
)]
#[cfg_attr(
    feature = "with-uuid",
    graphql(concrete(name = "UuidFilter", params(sea_orm::prelude::Uuid)))
)]
#[graphql(concrete(name = "BinaryFilter", params(BinaryVector)))]
#[graphql(concrete(name = "BooleanFilter", params(bool)))]
pub struct TypeFilter<T: async_graphql::InputType> {
    pub eq: Option<T>,
    pub ne: Option<T>,
    pub gt: Option<T>,
    pub gte: Option<T>,
    pub lt: Option<T>,
    pub lte: Option<T>,
    pub is_in: Option<Vec<T>>,
    pub is_not_in: Option<Vec<T>>,
    pub is_null: Option<bool>,
}

impl<T: async_graphql::InputType> FilterTrait for TypeFilter<T>
where
    T: async_graphql::InputType<RawValueType = T> + Clone,
    TypeFilter<T>: async_graphql::InputType<RawValueType = Self>
{
    type Ty = T;

    fn eq(&self) -> Option<Self::Ty> {
        self.eq.clone()
    }
    fn ne(&self) -> Option<Self::Ty> {
        self.ne.clone()
    }
    fn gt(&self) -> Option<Self::Ty> {
        self.gt.clone()
    }
    fn gte(&self) -> Option<Self::Ty> {
        self.gte.clone()
    }
    fn lt(&self) -> Option<Self::Ty> {
        self.lt.clone()
    }
    fn lte(&self) -> Option<Self::Ty> {
        self.lte.clone()
    }
    fn is_in(&self) -> Option<Vec<Self::Ty>> {
        self.is_in.clone()
    }
    fn is_not_in(&self) -> Option<Vec<Self::Ty>> {
        self.is_not_in.clone()
    }
    fn is_null(&self) -> Option<bool> {
        self.is_null.clone()
    }
    fn contains(&self) -> Option<String> {
        panic!("FilterType does not support contains")
    }
    fn starts_with(&self) -> Option<String> {
        panic!("FilterType does not support starts_with")
    }
    fn ends_with(&self) -> Option<String> {
        panic!("FilterType does not support ends_with")
    }
    fn like(&self) -> Option<String> {
        panic!("FilterType does not support like")
    }
    fn not_like(&self) -> Option<String> {
        panic!("FilterType does not support not_like")
    }
}

#[derive(Debug, Clone, async_graphql::InputObject)]
pub struct StringFilter {
    pub eq: Option<String>,
    pub ne: Option<String>,
    pub gt: Option<String>,
    pub gte: Option<String>,
    pub lt: Option<String>,
    pub lte: Option<String>,
    pub is_in: Option<Vec<String>>,
    pub is_not_in: Option<Vec<String>>,
    pub is_null: Option<bool>,
    pub contains: Option<String>,
    pub starts_with: Option<String>,
    pub ends_with: Option<String>,
    pub like: Option<String>,
    pub not_like: Option<String>,
}

impl FilterTrait for StringFilter {
    type Ty = String;

    fn eq(&self) -> Option<Self::Ty> {
        self.eq.clone()
    }
    fn ne(&self) -> Option<Self::Ty> {
        self.ne.clone()
    }
    fn gt(&self) -> Option<Self::Ty> {
        self.gt.clone()
    }
    fn gte(&self) -> Option<Self::Ty> {
        self.gte.clone()
    }
    fn lt(&self) -> Option<Self::Ty> {
        self.lt.clone()
    }
    fn lte(&self) -> Option<Self::Ty> {
        self.lte.clone()
    }
    fn is_in(&self) -> Option<Vec<Self::Ty>> {
        self.is_in.clone()
    }
    fn is_not_in(&self) -> Option<Vec<Self::Ty>> {
        self.is_not_in.clone()
    }
    fn is_null(&self) -> Option<bool> {
        self.is_null.clone()
    }
    fn contains(&self) -> Option<String> {
        self.contains.clone()
    }
    fn starts_with(&self) -> Option<String> {
        self.starts_with.clone()
    }
    fn ends_with(&self) -> Option<String> {
        self.ends_with.clone()
    }
    fn like(&self) -> Option<String> {
        self.like.clone()
    }
    fn not_like(&self) -> Option<String> {
        self.not_like.clone()
    }
}


pub trait FilterTrait: Sync + Send {
    type Ty: async_graphql::InputType;

    fn eq(&self) -> Option<Self::Ty>;
    fn ne(&self) -> Option<Self::Ty>;
    fn gt(&self) -> Option<Self::Ty>;
    fn gte(&self) -> Option<Self::Ty>;
    fn lt(&self) -> Option<Self::Ty>;
    fn lte(&self) -> Option<Self::Ty>;
    fn is_in(&self) -> Option<Vec<Self::Ty>>;
    fn is_not_in(&self) -> Option<Vec<Self::Ty>>;
    fn is_null(&self) -> Option<bool>;
    fn contains(&self) -> Option<String>;
    fn starts_with(&self) -> Option<String>;
    fn ends_with(&self) -> Option<String>;
    fn like(&self) -> Option<String>;
    fn not_like(&self) -> Option<String>;
}
