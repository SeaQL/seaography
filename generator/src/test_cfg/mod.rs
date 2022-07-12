use seaography_types::{ColumnMeta, ColumnType, RelationshipMeta, TableMeta};

pub fn get_tables_meta() -> Vec<TableMeta> {
    vec![
        get_char_table(),
        get_font_table()
    ]
}

pub fn get_char_table() -> TableMeta {
    TableMeta {
        table_name: "char".into(),
        columns: vec![
            ColumnMeta {
                col_name: "id".into(),
                not_null: true,
                is_primary_key: true,
                col_type: ColumnType::Integer32,
            },
            ColumnMeta {
                col_name: "character".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::String,
            },
            ColumnMeta {
                col_name: "size_w".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::Integer32,
            },
            ColumnMeta {
                col_name: "size_h".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::Integer32,
            },
            ColumnMeta {
                col_name: "font_id".into(),
                not_null: false,
                is_primary_key: false,
                col_type: ColumnType::Integer32,
            },
            ColumnMeta {
                col_name: "font_size".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::Integer32,
            },
        ],
        relations: vec![RelationshipMeta {
            src_table: "char".into(),
            dst_table: "font".into(),
            src_cols: vec![ColumnMeta {
                col_name: "font_id".into(),
                not_null: false,
                is_primary_key: false,
                col_type: ColumnType::Integer64,
            }],
            dst_cols: vec![ColumnMeta {
                col_name: "id".into(),
                not_null: true,
                is_primary_key: true,
                col_type: ColumnType::Integer64,
            }],
        }],
    }
}

pub fn get_font_table() -> TableMeta {
    TableMeta {
        table_name: "font".into(),
        columns: vec![
            ColumnMeta {
                col_name: "id".into(),
                not_null: true,
                is_primary_key: true,
                col_type: ColumnType::Integer32,
            },
            ColumnMeta {
                col_name: "name".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::String,
            },
            ColumnMeta {
                col_name: "language".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::Enum("LanguageEnum".into()),
            },
            ColumnMeta {
                col_name: "variant".into(),
                not_null: true,
                is_primary_key: false,
                col_type: ColumnType::Enum("VariantEnum".into()),
            },
        ],
        relations: vec![RelationshipMeta {
            src_table: "char".into(),
            dst_table: "font".into(),
            src_cols: vec![ColumnMeta {
                col_name: "font_id".into(),
                not_null: false,
                is_primary_key: false,
                col_type: ColumnType::Integer64,
            }],
            dst_cols: vec![ColumnMeta {
                col_name: "id".into(),
                not_null: true,
                is_primary_key: true,
                col_type: ColumnType::Integer64,
            }],
        }],
    }
}