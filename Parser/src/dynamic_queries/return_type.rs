use std::ops::Index;

use grdb_orm_lib::dyn_query::DynamicQuery;
use regex::Regex;
use sqlite_parser::{Metadata, Type};

use crate::configuration::Config;
use crate::line_writer::LineWriter;
use crate::some_kind_of_uppercase_first_letter;
use crate::swift_property::{
    create_row_index, create_swift_property, create_swift_type_name, decode_swift_property,
    is_build_in_type, wrap_null_check, SwiftPropertyDecoder,
};

/// Transforms the input of the toml return values in dyn_queries to usable Swift return types
pub struct ReturnType<'a> {
    pub dynamic_query: &'a DynamicQuery,
    pub line_writer: &'a mut LineWriter,
    pub tables: &'a Metadata,
    pub config: &'a Config,
}

/// The different ways of how rows should be decoded
#[derive(Clone, Debug)]
pub enum QuerySelectDecoding {
    // Custom decoding is not needed because a struct is generated with the correct decoding
    NotNeeded,
    // Decoding is needed, because a typealias is generated because just 1 column/type is selected
    Decoding(String),
}

pub enum Query {
    Select {
        return_type: String,
        decoding: QuerySelectDecoding,
    },
    // The query shouldn't start with 'insert', since that query is already generated
    UpdateOrDelete,
}

impl Query {
    pub fn return_type(&self) -> String {
        match &self {
            Query::Select {
                return_type,
                decoding: _,
            } => return_type.clone(),
            // This doesn't have a return type
            Query::UpdateOrDelete => "".to_string(),
        }
    }

    /// Replace the optional type here, no need for it.
    /// This is needed for the type to map, this is always nonnull.
    /// Only do this when there is a single type and it isn't an array, else e.g. (DbUser, SomeType?) will be corrupted
    pub fn replace_optional_for_closure(&self, return_types_is_array: bool) -> String {
        match &self {
            Query::Select {
                return_type,
                decoding: _,
            } => {
                if return_types_is_array {
                    return_type.clone()
                } else {
                    // The return type ALWAYS has a trailing question mark, remove it
                    assert!(return_type.ends_with('?'));

                    return_type.strip_suffix('?').unwrap().to_string()
                }
            }
            Query::UpdateOrDelete => panic!(),
        }
    }
}

impl<'a> ReturnType<'a> {
    pub fn parse(self) -> Query {
        if self.dynamic_query.return_types.is_empty() {
            return Query::UpdateOrDelete;
        }

        // The regex to check if the return type is table.column
        let regex_table_column = Regex::new(r"(.*)\.(.*)").unwrap();
        // The index of the row which should start decoding the next return value
        let mut index = 0;
        let mut decoding = vec![];
        let mut return_types_swift_struct = vec![];

        // Find out how to decode the return types
        for rt in self.dynamic_query.return_types.iter() {
            // Remove the optional type for now, since else the table can not be found in the table array
            let without_question_mark = rt.replace('?', "");
            // If the rt was an optional, make sure to append it back to without_question_mark if needed
            let suffix = if rt.contains('?') { "?" } else { "" };

            // If the type is a Swift type, it's easy, directly decode it
            // The Type is always TEXT, it doesn't matter for this case since blobs are handled in the last block
            if is_build_in_type(&without_question_mark, Type::Text) {
                let format = format!("row[{}]", index);

                // It always needs just 1 index
                index += 1;

                decoding.push(format);
                return_types_swift_struct
                    .push(create_swift_type_name(&without_question_mark, self.config) + suffix);
            } else if let Some(table) = self.tables.table(&without_question_mark) {
                // The return type is a table
                let swift_struct_name = create_swift_type_name(&without_question_mark, self.config);

                // Now the startingIndex initializer is useful
                let format = format!("{}(row: row, startingIndex: {})", swift_struct_name, index);
                let decode = wrap_null_check(!suffix.is_empty(), &index.to_string(), &format);

                // The index should be increased for the amount of columns
                index += table.columns.len();

                decoding.push(decode);
                return_types_swift_struct.push(swift_struct_name + suffix);
            } else {
                // This is a column of a table, e.g.: User.userUuid
                // Little harder to decode than the others
                // First, capture the table and column
                let captures = regex_table_column
                    .captures(&without_question_mark)
                    .unwrap_or_else(|| {
                        panic!("Can not find table.column for {}", &without_question_mark)
                    });

                // Now the column should be found
                let column = self
                    .tables
                    .table(captures.index(1))
                    .unwrap()
                    .column(captures.index(2))
                    .unwrap();

                // Transform the column to a Swift property
                let swift_property = create_swift_property(column, &self.config.custom_mapping);

                struct Decoder {
                    index: usize,
                }

                impl SwiftPropertyDecoder for Decoder {
                    fn row_index(&self) -> String {
                        self.index.to_string()
                    }

                    fn assign(&self, _property_name: &str, decoded: &str) -> String {
                        decoded.to_string()
                    }
                }

                let decoder = Decoder { index };

                // Find out how to decode this property
                let decoded = if let Some((_, _)) = swift_property.serialize_deserialize_blob(false)
                {
                    let row_index = create_row_index(&decoder.row_index());
                    let decode = format!(
                        "try! {}(serializedData: {})",
                        swift_property.swift_type.type_name, row_index
                    );
                    let decode = wrap_null_check(
                        swift_property.column.nullable || !suffix.is_empty(),
                        &decoder.row_index(),
                        &decode,
                    );

                    decoder.assign("", &decode)
                } else {
                    decode_swift_property(&decoder, &swift_property, !suffix.is_empty())
                };

                // Just decoding 1 column
                index += 1;

                decoding.push(decoded);
                return_types_swift_struct.push(swift_property.swift_type.type_name + suffix);
            }
        }

        assert_eq!(decoding.len(), return_types_swift_struct.len());

        let modifier = self.config.visibility.modifier();
        let return_type;
        let decoding_row;

        if return_types_swift_struct.len() == 1 {
            return_type = return_types_swift_struct[0].clone();

            // Decoding is always 1 row here
            decoding_row = QuerySelectDecoding::Decoding(decoding.remove(0));
        } else {
            let struct_name = format!(
                "{}Type",
                some_kind_of_uppercase_first_letter(&self.dynamic_query.func_name)
            );
            // Create a separate struct which holds the values
            let mut struct_properties = vec![];
            let mut initializer_row = vec![];

            for (index, s) in return_types_swift_struct.iter().enumerate() {
                let property_name = format!("gen{index}");

                struct_properties.push(format!("{modifier}let {property_name}: {s}"));
                initializer_row.push(format!("{property_name} = {}", decoding[index]));
            }

            let properties = struct_properties.join("\n");
            let initializer = initializer_row.join("\n");

            self.line_writer.add_line(format!(
                "struct {struct_name}: Equatable {{
                {properties}
                {modifier}init(row: Row) {{
                    {initializer}
                }}
            }}
            "
            ));

            decoding_row = QuerySelectDecoding::NotNeeded;
            return_type = struct_name;
        };

        // Make it an array or optional if needed
        let return_value = if self.dynamic_query.return_types_is_array {
            format!("[{return_type}]")
        } else {
            return_type + "?"
        };

        Query::Select {
            return_type: return_value,
            decoding: decoding_row,
        }
    }
}
