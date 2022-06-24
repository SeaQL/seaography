use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ColumnType {
    String,
    Integer8,
    Integer16,
    Integer32,
    Integer64,
    Unsigned8,
    Unsigned16,
    Unsigned32,
    Unsigned64,
    Float,
    Double,
    Json,
    Date,
    Time,
    DateTime,
    Timestamp,
    TimestampWithTimeZone,
    Decimal,
    Uuid,
    Binary,
    Boolean,
    Enum(String)
}
