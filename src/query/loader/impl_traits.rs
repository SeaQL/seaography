use super::*;

impl<T> PartialEq for KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.key
            .iter()
            .map(map_key)
            .eq(other.key.iter().map(map_key))
            && self.meta.eq(&other.meta)
    }
}

fn map_key(key: &sea_orm::Value) -> sea_orm::Value {
    match key {
        sea_orm::Value::TinyInt(value) => {
            let value: Option<i64> = value.map(|value| value as i64);
            sea_orm::Value::BigInt(value)
        }
        sea_orm::Value::SmallInt(value) => {
            let value: Option<i64> = value.map(|value| value as i64);
            sea_orm::Value::BigInt(value)
        }
        sea_orm::Value::Int(value) => {
            let value: Option<i64> = value.map(|value| value as i64);
            sea_orm::Value::BigInt(value)
        }
        sea_orm::Value::TinyUnsigned(value) => {
            let value: Option<u64> = value.map(|value| value as u64);
            sea_orm::Value::BigUnsigned(value)
        }
        sea_orm::Value::SmallUnsigned(value) => {
            let value: Option<u64> = value.map(|value| value as u64);
            sea_orm::Value::BigUnsigned(value)
        }
        sea_orm::Value::Unsigned(value) => {
            let value: Option<u64> = value.map(|value| value as u64);
            sea_orm::Value::BigUnsigned(value)
        }
        _ => key.clone(),
    }
}

impl<T> Eq for KeyComplex<T> where T: sea_orm::EntityTrait {}

impl<T> Hash for KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for key in self.key.iter() {
            match key {
                sea_orm::Value::TinyInt(value) => {
                    let value: Option<i64> = value.map(|value| value as i64);
                    value.hash(state);
                }
                sea_orm::Value::SmallInt(value) => {
                    let value: Option<i64> = value.map(|value| value as i64);
                    value.hash(state);
                }
                sea_orm::Value::Int(value) => {
                    let value: Option<i64> = value.map(|value| value as i64);
                    value.hash(state);
                }
                sea_orm::Value::TinyUnsigned(value) => {
                    let value: Option<u64> = value.map(|value| value as u64);
                    value.hash(state);
                }
                sea_orm::Value::SmallUnsigned(value) => {
                    let value: Option<u64> = value.map(|value| value as u64);
                    value.hash(state);
                }
                sea_orm::Value::Unsigned(value) => {
                    let value: Option<u64> = value.map(|value| value as u64);
                    value.hash(state);
                }
                _ => key.hash(state),
            }
        }
        self.meta.hash(state);
    }
}

impl<T> PartialEq for HashableGroupKey<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        self.rel_def.eq(&other.rel_def)
            && self.via_def.eq(&other.via_def)
            && self.filters.eq(&other.filters)
            && std::cmp::PartialEq::eq(
                &format!("{:?}", self.order_by),
                &format!("{:?}", other.order_by),
            )
    }
}

impl<T> Eq for HashableGroupKey<T> where T: sea_orm::EntityTrait {}

impl<T> Hash for HashableGroupKey<T>
where
    T: sea_orm::EntityTrait,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.rel_def.hash(state);
        self.via_def.hash(state);
        format!("{:?}", self.filters).hash(state);
        format!("{:?}", self.order_by).hash(state);
    }
}
