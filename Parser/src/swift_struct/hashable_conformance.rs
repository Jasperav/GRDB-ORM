use crate::table_meta_data::TableMetaData;

pub fn write(tmd: &mut TableMetaData) {
    tmd.line_writer
        .add_line("public func hash(into hasher: inout Hasher) {");

    for swift_property in tmd.primary_keys().into_iter().cloned().collect::<Vec<_>>() {
        tmd.line_writer.add_line(format!(
            "hasher.combine({})",
            swift_property.swift_property_name
        ));
    }

    tmd.line_writer.add_line("}");
}
