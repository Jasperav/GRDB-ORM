use crate::table_meta_data::TableMetaData;

/// Writes the conversion between the 'main struct' and the primary key struct
pub fn write_main_struct_to_pk(tmd: &mut TableMetaData<'_>) {
    tmd.line_writer
        .add_comment("Easy way to get the PrimaryKey from the table");

    let properties = tmd
        .primary_keys()
        .iter()
        .map(|pk| format!("{}: {}", pk.swift_property_name, pk.swift_property_name))
        .collect::<Vec<_>>()
        .join(", ");

    tmd.line_writer.add_with_modifier(format!(
        "func primaryKey() -> {} {{
            .init({})
        }}",
        tmd.primary_key_struct_name, properties
    ))
}
