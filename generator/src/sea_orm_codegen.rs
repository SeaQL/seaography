use sea_orm_codegen::WriterOutput;

use crate::{util::add_line_break, parser::parse_entity, writer::EntityDefinition};

pub fn generate_entities(
    table_crate_stmts: Vec<sea_query::TableCreateStatement>,
    expanded_format: bool,
) -> crate::Result<(Vec<EntityDefinition>, WriterOutput)> {
    let entity_writer = sea_orm_codegen::EntityTransformer::transform(table_crate_stmts)?;

    let entity_writer_ctx = sea_orm_codegen::EntityWriterContext::new(
        expanded_format,
        sea_orm_codegen::WithSerde::None,
        true,
        sea_orm_codegen::DateTimeCrate::Chrono,
        None,
    );

    let writer_output = entity_writer.generate(&entity_writer_ctx);

    let data: Vec<EntityDefinition> = writer_output
        .files
        .iter()
        .filter(|file| {
            file.name.ne(&"mod.rs".to_string())
                && file.name.ne(&"prelude.rs".to_string())
                && file.name.ne(&"sea_orm_active_enums.rs".to_string())
        })
        .map(|file| {
            parse_entity(file)
        })
        .collect();

    Ok((data, writer_output))
}

pub fn write_entities<P: AsRef<std::path::Path>>(
    path: &P,
    writer_output: WriterOutput,
) -> crate::Result<()> {
    for file in writer_output.files.iter() {
        let file_path = path.as_ref().join(file.name.clone());
        std::fs::write(file_path, add_line_break(file.content.parse().unwrap()))?;
    }

    Ok(())
}
