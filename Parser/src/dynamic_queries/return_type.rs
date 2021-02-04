use std::ops::Index;

use regex::Regex;
use sqlite_parser::Metadata;

use crate::configuration::Config;
use crate::swift_property::{
    create_swift_property, create_swift_type_name, decode_swift_property, is_build_in_type,
    SwiftPropertyDecoder,
};

/// Transforms the input of the toml return values in dyn_queries to usable Swift return types
pub struct ReturnType<'a> {
    pub return_types: &'a Vec<String>,
    pub return_type_is_array: bool,
    pub tables: &'a Metadata,
    pub config: &'a Config,
}

/// The result of calling `parse()` on [ReturnType]
pub struct ReturnTypeParsed {
    // The return type(s)
    pub return_type: String,
    // Information about how to decode the return_type(s)
    pub decoding: String,
}

impl<'a> ReturnType<'a> {
    pub fn parse(self) -> ReturnTypeParsed {
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
                if is_build_in_type(&without_question_mark) {
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
                        create_swift_type_name(&without_question_mark, &self.config);

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

                    // Find out how to decode this property
                    let decoded = decode_swift_property(&Decoder { index }, &swift_property);

                    // Just decoding 1 column
                    index += 1;

                    (decoded, swift_property.swift_type.type_name + suffix)
                }
            })
            .unzip();

        // Create the return value to return from query method
        let return_value = if self.return_type_is_array {
            assert!(!return_types_swift_struct.is_empty());

            let separated = return_types_swift_struct.join(", ");

            if return_types_swift_struct.len() == 1 {
                format!("[{}]", separated)
            } else {
                format!("[({})]", separated)
            }
        } else {
            assert_eq!(1, return_types_swift_struct.len());

            return_types_swift_struct[0].clone()
        };

        ReturnTypeParsed {
            return_type: return_value,
            decoding: format!("({})", decoding.join(", ")),
        }
    }
}
