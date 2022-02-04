use crate::table_meta_data::TableMetaData;

pub struct IdentifiableConformance<'a> {
    pub table_meta_data: &'a mut TableMetaData<'a>,
}

impl<'a> IdentifiableConformance<'a> {
    pub fn write(self) {
        let pk = self.table_meta_data.primary_keys();

        // Check if the properties contain an 'id' property
        // If that property is absent, create a new one for the Identifiable protocol
        let has_id_property = pk.iter().any(|p| p.swift_property_name == "id");
        let content = if has_id_property {
            // Nothing has to be in the body
            "".to_string()
        } else {
            let return_type = if pk.len() == 1 {
                pk[0].swift_type.type_name.clone()
            } else {
                "Int".to_string()
            };
            let id_type = if pk.len() == 1 {
                pk[0].swift_property_name.clone()
            } else {
                let mut hash = "var hasher = Hasher()\n\n".to_string();

                for p in &pk {
                    hash += &format!("hasher.combine({})\n", p.swift_property_name);
                }

                hash += "\nreturn hasher.finalize()";

                hash
            };

            format!(
                "public var id: {return_type} {{
                    {id_type}
            }}",
            )
        };

        self.table_meta_data.line_writer.add_line(format!(
            "extension {}: Identifiable {{
                {content}
        }}",
            self.table_meta_data.struct_name
        ));
    }
}
