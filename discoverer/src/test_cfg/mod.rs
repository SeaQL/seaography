use std::collections::HashMap;
use sea_schema::sea_query::{TableCreateStatement, Table, ColumnDef, ForeignKeyAction, ForeignKey, Value};
use sea_schema::sea_query::tests_cfg::{Char, Font};
use seaography_types::EnumMeta;
use crate::TablesHashMap;

pub fn get_tables_hash_map() -> TablesHashMap {
    let mut hashmap: TablesHashMap = HashMap::new();

    hashmap.insert("char".into(), get_char_table());
    hashmap.insert("font".into(), get_complex_fonts_table());

    hashmap.to_owned()
}

pub fn get_char_table() -> TableCreateStatement {
    Table::create()
        .table(Char::Table)
        .if_not_exists()
        .col(ColumnDef::new(Char::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Char::Character).string().not_null())
        .col(ColumnDef::new(Char::SizeW).integer().not_null())
        .col(ColumnDef::new(Char::SizeH).integer().not_null())
        .col(ColumnDef::new(Char::FontId).integer().default(Value::Int(None)))
        .col(ColumnDef::new(Char::FontSize).integer().not_null())
        .foreign_key(
            ForeignKey::create()
                .name("FK_2e303c3a712662f1fc2a4d0aad6")
                .from(Char::Table, Char::FontId)
                .to(Font::Table, Font::Id)
                .on_delete(ForeignKeyAction::Cascade)
                .on_update(ForeignKeyAction::Cascade)
        )
        .to_owned()
}

pub fn get_complex_fonts_table() -> TableCreateStatement {
    get_base_fonts_table()
        .col(ColumnDef::new(Font::Variant).enumeration("VariantEnum", vec!["Bold", "Italic", "Slim"]).not_null())
        .to_owned()
}

pub fn get_basic_fonts_table() -> TableCreateStatement {
    get_base_fonts_table()
        .col(ColumnDef::new(Font::Variant).integer().not_null())
        .to_owned()
}

pub fn get_base_fonts_table() -> TableCreateStatement {
    Table::create()
        .table(Font::Table)
        .if_not_exists()
        .col(ColumnDef::new(Font::Id).integer().not_null().auto_increment().primary_key())
        .col(ColumnDef::new(Font::Name).string().not_null())
        .col(ColumnDef::new(Font::Language).enumeration("LanguageEnum", vec!["English", "French", "German"]).not_null())
        .to_owned()
}

pub fn get_language_enum() -> EnumMeta {
    EnumMeta {
        enum_name: "LanguageEnum".into(),
        enum_values: vec!["English".into(), "French".into(), "German".into()],
    }
}

pub fn get_variant_enum() -> EnumMeta {
    EnumMeta {
        enum_name: "VariantEnum".into(),
        enum_values: vec!["Bold".into(), "Italic".into(), "Slim".into()],
    }
}