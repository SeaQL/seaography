use crate::util::add_line_break;

pub type EntityHashMap = std::collections::BTreeMap<String, proc_macro2::TokenStream>;

pub fn generate_entities(
    table_crate_stmts: Vec<seaography_discoverer::sea_schema::sea_query::TableCreateStatement>,
    expanded_format: bool,
) -> crate::Result<EntityHashMap> {
    let entity_writer = sea_orm_codegen::EntityTransformer::transform(table_crate_stmts)?;

    let entity_writer_ctx = sea_orm_codegen::EntityWriterContext::new(
        expanded_format,
        sea_orm_codegen::WithSerde::None,
        true,
        sea_orm_codegen::DateTimeCrate::Chrono,
        None,
    );

    let writer_output = entity_writer.generate(&entity_writer_ctx);

    let data: EntityHashMap = writer_output
        .files
        .iter()
        .map(|output_file| {
            (
                output_file.name.clone(),
                output_file.content.parse().unwrap(),
            )
        })
        .collect();

    Ok(data)
}

pub fn write_entities<P: AsRef<std::path::Path>>(
    path: &P,
    entities_hashmap: EntityHashMap,
) -> crate::Result<()> {
    for (name, content) in entities_hashmap.iter() {
        let file_path = path.as_ref().join(name);
        std::fs::write(file_path, add_line_break(content.clone()))?;
    }

    Ok(())
}
