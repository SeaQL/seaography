use super::*;

impl<T> PartialEq for KeyComplex<T>
where
    T: sea_orm::EntityTrait,
{
    fn eq(&self, other: &Self) -> bool {
        ValueTupleIter::new(&self.key)
            .map(map_key)
            .eq(ValueTupleIter::new(&other.key).map(map_key))
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
        for key in ValueTupleIter::new(&self.key) {
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

struct ValueTupleIter<'a> {
    key: &'a ValueTuple,
    index: usize,
}

impl<'a> ValueTupleIter<'a> {
    pub fn new(key: &'a ValueTuple) -> Self {
        Self { key, index: 0 }
    }
}

impl<'a> Iterator for ValueTupleIter<'a> {
    type Item = &'a Value;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.key {
            ValueTuple::One(iden1) => {
                if self.index == 0 {
                    Some(iden1)
                } else {
                    None
                }
            }
            ValueTuple::Two(iden1, iden2) => match self.index {
                0 => Some(iden1),
                1 => Some(iden2),
                _ => None,
            },
            ValueTuple::Three(iden1, iden2, iden3) => match self.index {
                0 => Some(iden1),
                1 => Some(iden2),
                2 => Some(iden3),
                _ => None,
            },
            ValueTuple::Many(vec) => vec.get(self.index),
            _ => unreachable!(),
        };
        self.index += 1;
        result
    }
}
