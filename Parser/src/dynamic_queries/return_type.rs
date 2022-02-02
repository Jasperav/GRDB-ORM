use std::ops::Index;

use regex::Regex;
use sqlite_parser::{Metadata, Type};

use crate::configuration::Config;
use crate::line_writer::LineWriter;
use crate::swift_property::{
    create_row_index, create_swift_property, create_swift_type_name, decode_swift_property,
    is_build_in_type, wrap_null_check, SwiftPropertyDecoder,
};

/// Transforms the input of the toml return values in dyn_queries to usable Swift return types
pub struct ReturnType<'a> {
    pub return_types: &'a Vec<String>,
    pub return_type_is_array: bool,
    pub tables: &'a Metadata,
    pub config: &'a Config,
}

pub enum Query {
    Select {
        return_type: String,
        // Information about how to decode the return_type(s)
        decoding: String,
    },
    // The query shouldn't start with 'insert', since that query is already generated
    UpdateOrDelete,
}

impl Query {
    /// Writes a Swift type alias for a select query return type, which makes it easy for the user to reuse the type
    pub fn write_type_alias(
        &self,
        line_writer: &mut LineWriter,
        capitalized_func_name: &str,
    ) -> (String, String) {
        match &self {
            Query::Select {
                return_type,
                decoding: _decoding,
            } => {
                let type_alias_name = format!("{}Type", capitalized_func_name);

                line_writer.add_line(format!("typealias {} = {}", type_alias_name, &return_type));

                (type_alias_name, return_type.to_string())
            }
            Query::UpdateOrDelete => {
                // Don't do anything
                ("".to_string(), "".to_string())
            }
        }
    }

    pub fn return_type(&self) -> String {
        match &self {
            Query::Select {
                return_type,
                decoding: _decoding,
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
                decoding: _decoding,
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
        if self.return_types.is_empty() {
            return Query::UpdateOrDelete;
        }

        // The regex to check if the return type is table.column
        let regex_table_column = Regex::new(r"(.*)\.(.*)").unwrap();
        // The index of the row which should start decoding the next return value
        let mut index = 0;

        let (decoding, return_types_swift_struct): (Vec<_>, Vec<_>) = self
            .return_types
            .iter()
            .map(|rt| {
                // Remove the optional type for now, since else the table can not be found in the table array
                let without_question_mark = rt.replace("?", "");
                // If the rt was an optional, make sure to append it back to without_question_mark if needed
                let suffix = if rt.contains('?') { "?" } else { "" };

                // If the type is a Swift type, it's easy, directly decode it
                // The Type is always TEXT, it doesn't matter for this case since blobs are handled in the last block
                if is_build_in_type(&without_question_mark, Type::Text) {
                    let format = format!("row[{}]", index);

                    // It always needs just 1 index
                    index += 1;

                    (
                        format,
                        create_swift_type_name(&without_question_mark, self.config) + suffix,
                    )
                } else if let Some(table) = self.tables.table(&without_question_mark) {
                    // The return type is a table
                    let swift_struct_name =
                        create_swift_type_name(&without_question_mark, self.config);

                    // Now the startingIndex initializer is useful
                    let format =
                        format!("{}(row: row, startingIndex: {})", swift_struct_name, index);

                    // The index should be increased for the amount of columns
                    index += table.columns.len();

                    (format, swift_struct_name + suffix)
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
                    let decoded =
                        if let Some((_, _)) = swift_property.serialize_deserialize_blob(false) {
                            let row_index = create_row_index(&decoder.row_index());
                            let decode = format!(
                                "try! {}(serializedData: {})",
                                swift_property.swift_type.type_name, row_index
                            );
                            let decode = wrap_null_check(
                                swift_property.column.nullable,
                                &decoder.row_index(),
                                &decode,
                            );

                            decoder.assign("", &decode)
                        } else {
                            decode_swift_property(&decoder, &swift_property)
                        };

                    // Just decoding 1 column
                    index += 1;

                    (decoded, swift_property.swift_type.type_name + suffix)
                }
            })
            .unzip();

        assert!(!return_types_swift_struct.is_empty());

        // Create the return value to return from query method
        let return_value = if self.return_type_is_array {
            let separated = return_types_swift_struct.join(", ");

            if return_types_swift_struct.len() == 1 {
                format!("[{}]", separated)
            } else {
                format!("[({})]", separated)
            }
        } else if return_types_swift_struct.len() == 1 {
            let rt = return_types_swift_struct[0].to_string();

            format!("{}?", rt)
        } else {
            format!("({})?", return_types_swift_struct.join(", "))
        };

        Query::Select {
            return_type: return_value,
            decoding: format!("({})", decoding.join(", ")),
        }
    }
}
